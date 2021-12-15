use rocket::{fs::FileServer, response::Redirect};

#[macro_use] extern crate rocket;

// static

// Index
#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/public/index.html"))
}

#[get("/favicon.ico")]
fn favicon() -> Redirect {
    Redirect::to(uri!("/public/favicon.ico"))
}

// 404

// About:

// Home:

// Tags:

// Blogs:

// Login:

// UpMK:

// TODO


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![favicon, index])
        .mount("/public", FileServer::from("static"))
}


