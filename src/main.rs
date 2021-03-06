mod about;
mod blogs;
mod database;
mod markdown;
mod tags;
mod user;

use crate::about::about_route;
use crate::blogs::{add_blog, delet_blog_by_id, get_blog_by_id, get_blog_list, update_blog};
use crate::database::hello;
use database::DbFairing;
use rocket::{fs::FileServer, response::Redirect};

#[macro_use]
extern crate rocket;

// [x] static

// [x] Index
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

// [x] Home:

// [x] About:

// [ ] Tags:

// [ ] Blogs:

// [ ] Login:

// [ ] UpMK:

// TODO

// #[rocket::main]
// async fn main() -> Result<(), rocket::Error> {
//     rocket::build()
//         .mount("/", routes![favicon, index, about_route, hello])
//         .mount("/home", routes![index])
//         .mount("/public", FileServer::from("static"))
//         .manage(Database::new("db/sqlx/db.sqlite").await.unwrap())
//         .launch()
//         .await
// }

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(DbFairing::with_name("database"))
        .mount("/", routes![favicon, index, about_route, hello])
        .mount("/home", routes![index])
        .mount(
            "/blog",
            routes![
                add_blog,
                delet_blog_by_id,
                update_blog,
                get_blog_by_id,
                get_blog_list
            ],
        )
        .mount("/public", FileServer::from("static"))
}
