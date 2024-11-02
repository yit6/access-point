use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::fmt;
use crate::map_geo::*;

// An unsimplified AccessPoint type
#[derive(Debug, EnumIter)]
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
pub struct AccessPointType(RawAccessPointType);

pub enum AccessPointStatus {
	Working,
	InRepair,
	NotWorking,
	Info(String),
	WarningInfo(String),
}

pub struct AccessPoint {
	kind: AccessPointType,
	location: Location,
	status: AccessPointStatus,
}


pub struct AccessPoints {
	points: Vec<AccessPoint>
}