use std::{
	fmt, 
	collections::HashMap,
};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use serde::{Serialize, Deserialize};

use google_maps::geocoding::response::geocoding::Geocoding; // blegh

use rocket::{http::Status, State, fairing::AdHoc};

pub fn stage() -> AdHoc {
	AdHoc::on_ignite("AccessPoints", |rocket| async {
		rocket
			.manage(AccessPoints::new())
			.mount("/ap", routes![get_ap])
	})
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
	status: AccessPointStatus,
}

impl AccessPoint {
	pub fn set_status(&mut self, status: &AccessPointStatus) {
		self.status = *status;
	}
}

pub type APID = usize;

#[derive(Serialize, Deserialize)]
pub struct AccessPoints {
	pub points: HashMap<APID, AccessPoint>,
}

impl AccessPoints {
	pub fn new() -> Self {
		AccessPoints {
			points: HashMap::new()
		}
	}
}

#[get("/<id>")]
fn get_ap(id: APID, group: &State<AccessPoints>) -> (Status, Option<String>) {
	let _point = group.points.get(&id);
	let point = match _point {
		Some(n) => Some(serde_json::to_string(n).unwrap()),
		None => None,
	};
	(match point {Some(_) => Status::Accepted, None => Status::NotFound}, point)
}