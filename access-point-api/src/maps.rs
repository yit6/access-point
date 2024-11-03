use google_maps::prelude::GoogleMapsClient;
use rocket::fairing::AdHoc;

/*pub fn stage() -> AdHoc {
	AdHoc::on_ignite("GoogleMaps", |rocket| async {
		rocket
			.manage(GoogleMapsClient::new("API-KEY"))
	})
}*/