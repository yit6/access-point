//! Store the state and handle API endpoints for user accounts
use std::{collections::{HashMap, HashSet}, fs::{self, File}, io::Write, path::Path, sync::{Arc, Mutex}};

use rocket::{fairing::AdHoc, http::{ContentType, Status}, serde::{json::Json, Deserialize}, State};
use pwhash::bcrypt;
use serde::Serialize;

use crate::ap::APID;

const USERS_FILE: &str = "data/users.json";

/// Create an AdHoc fairing to add the routes and manage the state.
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Users", |rocket| async {
        rocket
            .manage(Users::load(Path::new(USERS_FILE)).unwrap_or(Users::new()))
            .mount("/user", routes![
                add_access_point,
                create_user,
                get_user,
            ]).attach(AdHoc::on_shutdown("Users", |rocket| Box::pin(async {
                rocket.state::<Users>().unwrap().save(&mut File::create(USERS_FILE).expect("Failed to open user file")).expect("Failed to save users");
            })))
    })
}

/// Represent a user account.
/// Has a username, hashed password and a set of access_points.
#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[get("/<username>")]
fn get_user(username: String, users: &State<Users>) -> (Status, Option<String>) {
    let user = users.get(username);
    if user.is_none() { return (Status::NotFound, None); }
    let user = serde_json::to_string(&user.unwrap());
    if user.is_err() { return (Status::NotFound, None); }
    (Status::Ok, user.ok())
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

    /// Load user data from a JSON file
    pub fn load(path: &Path) -> Option<Self> {
        let users: HashMap<String, User> = serde_json::from_str(&fs::read_to_string(path).ok()?).ok()?;
        Some(Users {
            users: Arc::new(Mutex::new(users)),
        })
    }

    /// Save user data into a JSON file
    pub fn save(&self, file: &mut File) -> Result<(),std::io::Error> {
        let mut users: HashMap<String, User> = HashMap::new();
        
        // TODO: This is terrible
        for (name,user) in Arc::clone(&self.users).lock().unwrap().iter() {
            users.insert(name.to_string(), user.clone());
        }
        file.write_all(serde_json::to_string(&users)?.as_bytes())
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

    fn get(&self, username: String) -> Option<User> {
        let users = Arc::clone(&self.users);
        let users = users.lock().unwrap();
        let user = users.get(&username)?;
        return Some(user.clone());
    }

    pub fn get_users_with_access_point(&self, id: APID) -> Vec<String> {
        let users = Arc::clone(&self.users);
        let users = users.lock().unwrap();
        users.values().filter_map(|user| {
            if user.access_points.contains(&id) {
                Some(user.username.clone())
            } else {
                None
            }
        }).collect()
    }
}
