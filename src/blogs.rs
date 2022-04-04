use rocket::response::Redirect;

#[get("/blog/article/<name>")]
fn markdown_request(name: String) -> Redirect {
    Redirect::to(format!("/public/markdown/{}.md", name))
}
