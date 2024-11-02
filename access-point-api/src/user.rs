use std::{collections::{HashMap, HashSet}, sync::{Arc, Mutex}};

use rocket::{fairing::AdHoc, http::Status, serde::{json::Json, Deserialize}, State};
use pwhash::bcrypt;

use crate::ap::APID;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Users", |rocket| async {
        rocket
            .manage(Users::new())
            .mount("/user", routes![
                add_access_point,
                create_user,
            ])
    })
}

#[derive(Debug)]
struct User {
    username: String,
    password: String,
    access_points: HashSet<APID>,
}

impl User {
    pub fn new(username: String, password: String) -> Option<Self> {
        Some(User {
            username,
            password: bcrypt::hash(password).ok()?,
            access_points: HashSet::new(),
        })
    }
}

#[derive(Debug, Deserialize)]
struct DataCreateUser {
    username: String,
    password: String,
}

#[post("/", data="<input>")]
fn create_user(input: Json<DataCreateUser>, users: &State<Users>) -> Status{
    let user = User::new(input.username.to_string(), input.password.to_string());
    if let Some(user) = user {
        println!("{:?}",user);
        println!("{:?}",users);
        users.add(user);
        Status::Ok
    } else {
        Status::InternalServerError
    }
}

#[derive(Debug, Deserialize)]
struct DataAddAccessPoint {
    username: String,
    access_point: APID,
}

#[post("/add", data="<input>")]
fn add_access_point(input: Json<DataAddAccessPoint>, users: &State<Users>) -> Status {
    match users.add_access_point(&input.username, input.access_point) {
        Ok(_) => Status::Ok,
        Err(_) => Status::NotFound,
    }
}

#[derive(Debug)]
pub struct Users {
    users: Arc<Mutex<HashMap<String,User>>>,
}

impl Users {
    pub fn new() -> Self {
        Users {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn add(&self, user: User) {
        let users = Arc::clone(&self.users);
        let mut users = users.lock().unwrap();
        users.insert(user.username.clone(), user);
    }

    fn add_access_point(&self, username: &String, access_point: APID) -> Result<(), ()> {
        let users = Arc::clone(&self.users);
        let mut users = users.lock().unwrap();

        if let Some(user) = users.get_mut(username) {
            user.access_points.insert(access_point);
            return Ok(());
        }
        Err(())
    }
}
