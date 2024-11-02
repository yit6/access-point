use std::collections::HashMap;

use rocket::serde::{Deserialize, json::Json};
use pwhash::bcrypt;

#[derive(Debug)]
struct User {
    username: String,
    password: String,
}

impl User {
    pub fn new(username: String, password: String) -> Option<User> {
        Some(User {
            username,
            password: bcrypt::hash(password).ok()?,
        })
    }
}

#[derive(Debug, Deserialize)]
struct DataCreateUser<'a> {
    username: &'a str,
    password: &'a str,
}

#[post("/user", data="<input>")]
pub fn create_user(input: Json<DataCreateUser<'_>>) {
    let user = User::new(input.username.to_string(), input.password.to_string());
    println!("{:?}",user);
}
