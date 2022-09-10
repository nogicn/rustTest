use rocket::serde::Deserialize;
use rocket::serde::{Serialize, json::Json};
use rocket::response::content;

#[macro_use] extern crate rocket;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]pub struct User {
    pub(crate) id: u16,
    pub(crate) age: u16,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) framework: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]pub struct Users {
    pub(crate) users: Vec<User>
}


#[get("/")]
fn index() -> Json<Users> {
    let mut user = Vec::with_capacity(1000);
    for index in 1..1001 {
        user.push(User {
            id: index,
            age: 25,
            first_name: format!("First_Name{}", index),
            last_name: format!("Last_Name{}", index),
            framework: "Rust (Warp)".to_owned(),
        })
    }
    let mut users: Users = Users { users: (user) };
    Json(users)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
