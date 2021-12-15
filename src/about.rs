use std::io::Cursor;

use rocket::{
    http::ContentType,
    response::{content::Json, Responder},
    Response,
};
use serde::{Deserialize, Serialize};

#[get("/About")]
pub fn about_route() -> Json<AboutInfor> {
    let aboutme = AboutInfor {
        user: String::from("Sid"),
        quote: String::from("Well, you are right right right."),
        email: String::from("SidBayeck@outlook.com"),
        github: String::from("https://github.com/XaydBayeck"),
    };

    Json(aboutme)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AboutInfor {
    user: String,
    quote: String,
    email: String,
    github: String,
}

impl<'r> Responder<'r, 'static> for AboutInfor {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let infor_string = serde_json::to_string(&self).unwrap();
        Response::build()
            .sized_body(infor_string.len(), Cursor::new(infor_string))
            .header(ContentType::JSON)
            .ok()
    }
}
