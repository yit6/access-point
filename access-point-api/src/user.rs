use std::{collections::HashMap, sync::{Arc, Mutex}};

use rocket::{serde::{json::Json, Deserialize}, State};
use pwhash::bcrypt;

#[derive(Debug)]
struct User {
    username: String,
    password: String,
}

impl User {
    pub fn new(username: String, password: String) -> Option<Self> {
        Some(User {
            username,
            password: bcrypt::hash(password).ok()?,
        })
    }
}

#[derive(Debug, Deserialize)]
struct DataCreateUser {
    username: String,
    password: String,
}

#[post("/user", data="<input>")]
pub fn create_user(input: Json<DataCreateUser>, users: &State<Users>) {
    let user = User::new(input.username.to_string(), input.password.to_string());
    if let Some(user) = user {
        println!("{:?}",user);
        println!("{:?}",users);
        users.add(user);
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

    pub fn add(&self, user: User) {
        let users = Arc::clone(&self.users);
        let mut users = users.lock().unwrap();
        users.insert(user.username.clone(), user);
    }
}
