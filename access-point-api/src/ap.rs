use std::fmt;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use serde::{Serialize, Deserialize};

use crate::map_geo::*;

// An unsimplified AccessPoint type
#[derive(Debug, EnumIter, Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub struct AccessPointType(RawAccessPointType);

#[derive(Serialize, Deserialize)]
pub enum AccessPointStatus {
	Working,
	InRepair,
	NotWorking,
	Info(String),
	WarningInfo(String),
}

#[derive(Serialize, Deserialize)]
pub struct AccessPoint {
	kind: AccessPointType,
	location: Location,
	status: AccessPointStatus,
}


#[derive(Serialize, Deserialize)]
pub struct AccessPoints {
	points: Vec<AccessPoint>,
}