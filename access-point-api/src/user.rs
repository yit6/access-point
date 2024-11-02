//! Store the state and handle API endpoints for user accounts
use std::{collections::{HashMap, HashSet}, sync::{Arc, Mutex}};

use rocket::{fairing::AdHoc, http::Status, serde::{json::Json, Deserialize}, State};
use pwhash::bcrypt;

use crate::ap::APID;

/// Create an AdHoc fairing to add the routes and manage the state.
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

/// Represent a user account.
/// Has a username, hashed password and a set of access_points.
#[derive(Debug)]
pub struct User {
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
fn create_user(input: Json<DataCreateUser>, users: &State<Users>) -> Status {
    let user = User::new(input.username.to_string(), input.password.to_string());
    if let Some(user) = user {
        println!("{:?}",user);
        println!("{:?}",users);
        match users.add(user) {
            Ok(_) => Status::Ok,
            Err(_) => Status::Conflict,
        }
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

/// Store list of [`User`]s
#[derive(Debug)]
pub struct Users {
    users: Arc<Mutex<HashMap<String,User>>>,
}

impl Users {

    /// Create an new [`Users`] object with no [`User`]s
    pub fn new() -> Self {
        Users {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create a new [`User`].
    /// Returns `Ok(())` on success and `Err(())` if there is a conflict.
    fn add(&self, user: User) -> Result<(),()> {
        let users = Arc::clone(&self.users);
        let mut users = users.lock().unwrap();
        if users.contains_key(&user.username) { return Err(()); }
        users.insert(user.username.clone(), user);
        Ok(())
    }

    /// Add an access point into a [`User`] account.
    /// Returns `Ok(())` on success and `Err(())` if the user doesn't exist.
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
