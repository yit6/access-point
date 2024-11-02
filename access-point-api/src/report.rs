use crate::ap::*;

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
	pub fn fulfill(&self, group: &mut AccessPoints) -> Result<(), ()> {
		group.points.get_mut(&self.point).ok_or(())?
			.set_status(&self.status_change);
		Ok(())
	}
}