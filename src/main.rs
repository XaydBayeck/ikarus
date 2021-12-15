mod about;
mod blogs;
mod markdown;
mod tags;
mod user;

use crate::about::about_route;
use rocket::{fs::FileServer, response::Redirect};

#[macro_use]
extern crate rocket;

// static

// Index
#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/public/index.html"))
}

// favicon
#[get("/favicon.ico")]
fn favicon() -> Redirect {
    Redirect::to(uri!("/public/favicon.ico"))
}

// TODO 404Page

// Home:

// About:

// Tags:

// Blogs:

// Login:

// UpMK:

// TODO

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![favicon, index, about_route])
        .mount("/home", routes![index])
        .mount("/public", FileServer::from("static"))
}
