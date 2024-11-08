use std::{
    collections::HashMap, fmt, fs::{self, File}, io::Write, path::Path, sync::{Arc, Mutex}
};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use serde::{Serialize, Deserialize};

use rocket::{
    fairing::AdHoc, 
    http::Status, 
    response::status, 
    State, 
    serde::json::Json
};

use crate::user::Users;

const POINTS_FILE: &str = "data/points.json";

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("AccessPoints", |rocket| async {
        let points = AccessPoints::load(Path::new(POINTS_FILE)).unwrap_or(AccessPoints::new());
        rocket
            .manage(points)
            .mount("/ap", routes![
                create_access_point,
                get_ap,
                get_group,
                report_issue,
            ])
            .attach(AdHoc::on_shutdown("Users", |rocket| Box::pin(async {
                rocket.state::<AccessPoints>().unwrap().save(&mut File::create(POINTS_FILE).expect("Failed to open points file")).expect("Failed to save points")
            })))
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    lat: f32,
    long: f32,
}

#[allow(dead_code)]
pub struct Report {
    pub point: APID,
    description: Option<String>,
    status_change: AccessPointStatus,
}

impl Report {
    pub fn new(point: APID) -> Self {
        Report {
            point, description: None,
            status_change: AccessPointStatus::default(),
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_status(mut self, status: AccessPointStatus) -> Self {
        self.status_change = status;
        self
    }

    // change the result
    pub fn fulfill(&self, group: &State<AccessPoints>, users: &State<Users>) -> Result<(), ()> {
        group.set_status(self.point, self.status_change);
        users.inform_users(self.point, self.status_change);
        Ok(())
    }
}


// An unsimplified AccessPoint type
#[derive(Debug, Clone, EnumIter, Serialize, Deserialize)]
enum RawAccessPointType {
    Wheelchair,
    Interpreter,
    // Add others here...
    Any(String),
}

impl fmt::Display for RawAccessPointType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl RawAccessPointType {
    fn simplify(self) -> Self {
        match self {
            RawAccessPointType::Any(string) => {
                let mut result = RawAccessPointType::Any(string.clone());
                for variant in RawAccessPointType::iter() {
                    if string == variant.to_string() {
                        result = variant;
                    }
                }
                result
            },
            other => other,
        }
    }
}


// A guaranteed simplified AccessPoint type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPointType(RawAccessPointType);

impl ToString for AccessPointType {
    fn to_string(&self) -> String {
        match &self.0 {
            RawAccessPointType::Wheelchair => "Wheelchair service".to_string(),
            RawAccessPointType::Interpreter => "Language interpreter".to_string(),
            RawAccessPointType::Any(s) => if s == "" { "Miscellaneous".to_string() } else { s.to_string() },
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub enum AccessPointStatus {
    Working,
    InRepair,
    #[default]
    NotWorking,
}

impl ToString for AccessPointStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Working => "in working order!".to_string(),
            Self::InRepair => "being repaired!".to_string(),
            Self::NotWorking => "not currently operational.".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPoint {
    id: Option<APID>,
    name: String,
    kind: AccessPointType,
    pub nice_kind: String,
    location: Location,
    pub status: AccessPointStatus,
    pub nice_status: String,
}

impl AccessPoint {
    pub fn from_lat_long(lat: f32, long: f32) -> Self {
        AccessPoint {
            kind: AccessPointType(RawAccessPointType::Any("".to_string())),
            name: format!("Access point at {}, {}", lat, long),
            location: Location { lat, long },
            status: AccessPointStatus::NotWorking,
            id: None,
            nice_status: AccessPointStatus::NotWorking.to_string(),
            nice_kind: AccessPointType(RawAccessPointType::Any("".to_string())).to_string(),
        }
    }

    pub fn with_type(mut self, kind: AccessPointType) -> Self {
        self.kind = kind.clone();
        self.nice_kind = kind.to_string();
        self
    }

    pub fn with_status(mut self, status: AccessPointStatus) -> Self {
        self.status = status;
        self.nice_status = status.to_string();
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_id(mut self, id: APID) -> Self {
        self.id = Some(id);
        self
    }
}

pub type APID = usize;

#[derive(Debug)]
pub struct AccessPoints {
    pub points: Arc<Mutex<HashMap<APID, AccessPoint>>>,
}

#[derive(Debug, Responder, Serialize, Deserialize)]
struct AccessPointsSerDe {
    pub points: String,
}

impl AccessPointsSerDe {
    pub fn from_group(group: &State<AccessPoints>) -> Self {
        let points = Arc::clone(&group.points);
        let points = points.lock().unwrap();
        let points = serde_json::to_string(&points.clone()).unwrap();
        AccessPointsSerDe {
            points,
        }
    }
}

impl AccessPoints {
    pub fn new() -> Self {
        AccessPoints {
            points: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn next_id(&self) -> APID {
        let points = Arc::clone(&self.points);
        let points = points.lock().unwrap();
        if points.keys().len() == 0 {
            0
        } else {
            points.keys().max().unwrap() + 1
        }
    }

    pub fn add_access_point(&self, access_point: AccessPoint) -> Status {
        let points = Arc::clone(&self.points);
        let id = self.next_id();
        let mut points = points.lock().unwrap();
        let access_point = access_point.with_id(id);
        points.insert(id, access_point.clone());
        Status::Ok
    }

    pub fn load(path: &Path) -> Option<Self> {
        let access_points: HashMap<APID, AccessPoint> = serde_json::from_str(&fs::read_to_string(path).ok()?).ok()?;

        Some(AccessPoints {
            points: Arc::new(Mutex::new(access_points)),
        })
    }

    pub fn save(&self, file: &mut File) -> Result<(),std::io::Error> {
        let mut access_points: HashMap<APID, AccessPoint> = HashMap::new();

        // TODO: This is terrible
        for (apid,access_point) in Arc::clone(&self.points).lock().unwrap().iter() {
            access_points.insert(*apid, access_point.clone());
        }
        file.write_all(serde_json::to_string(&access_points)?.as_bytes())
    }

    pub fn get_ap(&self, id: APID) -> Option<AccessPoint> {
        let points = Arc::clone(&self.points);
        let points = points.lock().unwrap();
        points.get(&id).cloned()
    }

    pub fn set_status(&self, id: APID, status: AccessPointStatus) {
        let points = Arc::clone(&self.points);
        let mut points = points.lock().unwrap();
        points.get_mut(&id).unwrap().status = status;
    }
}

#[get("/<id>")]
fn get_ap(id: APID, group: &State<AccessPoints>) -> (Status, Option<String>) {
    let _point = group.get_ap(id);
    let point = match _point {
        Some(n) => Some(serde_json::to_string(&n).unwrap()),
        None => None,
    };
    (match point {Some(_) => Status::Ok, None => Status::NotFound}, point)
}

#[get("/")]
fn get_group(group: &State<AccessPoints>) -> (Status, AccessPointsSerDe) {
    (Status::Ok, AccessPointsSerDe::from_group(group))
}

#[put("/issue/<id>")]
fn report_issue(id: APID, group: &State<AccessPoints>, users: &State<Users>) -> Status {
    Report::new(id)
        .with_status(AccessPointStatus::NotWorking)
        .fulfill(group, users);

    Status::Ok
}

#[derive(Debug, Deserialize)]
struct DataCreateAccessPoint {
    name: Option<String>,
    status: Option<AccessPointStatus>,
    kind: Option<AccessPointType>,
    lat: f32,
    long: f32,
}

#[post("/", data="<input>")]
fn create_access_point(input: Json<DataCreateAccessPoint>, group: &State<AccessPoints>) -> Status {
    let mut point = AccessPoint::from_lat_long(input.lat, input.long);

    if let Some(name) = input.name.clone() { point = point.with_name(name); }
    if let Some(status) = input.status { point = point.with_status(status); }
    if let Some(kind) = input.kind.clone() { point = point.with_type(kind); }

    group.add_access_point(point);
    Status::Created
}
