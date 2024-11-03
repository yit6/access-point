// TODO

use rocket::fairing::AdHoc;

use crate::ap::Report;
use crate::user::{User, Users};

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Notifications", |rocket| async {
        rocket
            .manage(NotificationService::new())
            .mount("/notifications", routes![
                add_access_point,
                create_user,
            ]).attach(AdHoc::on_shutdown("Users", |rocket| Box::pin(async {
                rocket.state::<Users>().unwrap().save(&mut File::create(USERS_FILE).expect("Failed to open user file")).expect("Failed to save users");
            })))
    })
}

pub struct Notification {
	report: Report,
}

pub struct NotificationService {
	users: Vec<String>,
}

impl NotificationService {
	pub fn new() -> Self {
		NotificationService {
			users: vec![],
		}
	}
}

#[]