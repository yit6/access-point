use std::{
	fmt, 
	collections::HashMap,
	sync::{Arc, Mutex}
};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use serde::{Serialize, Deserialize};

use google_maps::geocoding::response::geocoding::Geocoding; // blegh

use rocket::{response::status, http::Status, State, fairing::AdHoc};

pub fn stage() -> AdHoc {
	AdHoc::on_ignite("AccessPoints", |rocket| async {
		rocket
			.manage(AccessPoints::new())
			.mount("/ap", routes![get_ap])
	})
}

#[allow(dead_code)]
pub struct Report {
	point: APID,
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
	pub fn fulfill(&self, group: &State<AccessPoints>) -> Result<(), ()> {
		group.set_status(self.point, self.status_change);
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

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub enum AccessPointStatus {
	Working,
	InRepair,
	#[default]
	NotWorking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPoint {
	kind: AccessPointType,
	location: Geocoding,
	pub status: AccessPointStatus,
}

pub type APID = usize;

#[derive(Debug)]
pub struct AccessPoints {
	pub points: Arc<Mutex<HashMap<APID, AccessPoint>>>,
}

impl AccessPoints {
	pub fn new() -> Self {
		AccessPoints {
			points: Arc::new(Mutex::new(HashMap::new())),
		}
	}

	pub fn get_ap(&self, id: APID) -> Option<AccessPoint> {
		let points = Arc::clone(&self.points);
        let points = points.lock().unwrap();
        Some(points.get(&id).unwrap().clone())
	}

	pub fn set_status(&self, id: APID, status: AccessPointStatus) {
		let points = Arc::clone(&self.points);
        let mut points = points.lock().unwrap();
        points.get_mut(&id).unwrap().status = status;
	}
}

#[get("/<id>", rank = 2)]
fn get_ap(id: APID, group: &State<AccessPoints>) -> (Status, Option<String>) {
	let _point = group.get_ap(id);
	let point = match _point {
		Some(n) => Some(serde_json::to_string(&n).unwrap()),
		None => None,
	};
	(match point {Some(_) => Status::Accepted, None => Status::NotFound}, point)
}

#[put("/issue/<id>")]
fn report_issue(id: APID, group: &State<AccessPoints>) -> status::Accepted<()> {
	Report::new(id)
		.with_status(AccessPointStatus::NotWorking)
		.fulfill(group);
	status::Accepted(())
}