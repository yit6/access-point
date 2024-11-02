use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Location {
	pub latitude: f32,
	pub longitude: f32,
}