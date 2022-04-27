use rocket::{http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::database::Database;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    id: i32,
    name: String,
    admin: bool,
    password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Regist {
    name: String,
    password: Option<String>,
}

#[post("/regist", format = "json", data = "<reg>")]
pub fn regist(reg: Json<Regist>, pool: &State<Database>) -> Status {
    todo!()
}

#[delete("/delet/<id>")]
pub fn delet(id: i32, pool: &State<Database>) -> Status {
    todo!()
}

#[post("/modify", format = "json", data = "<user>")]
pub fn update(user: Json<User>, pool: &State<Database>) -> Status {
    todo!()
}
