use rocket::{http::Status, response::Redirect, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::database::Database;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Blog {
    pub id: i32,
    pub title: String,
    pub tags: Option<String>,
    pub create_time: String,
    pub update_time: Option<String>,
    pub body: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct BlogInfo {
    pub id: i32,
    pub title: String,
    pub tags: Option<String>,
    pub create_time: String,
}

impl Blog {
    pub async fn insert(&self, pool: &SqlitePool) -> sqlx::Result<()> {
        sqlx::query("insert into blog values ($1, $2, $3, $4, $5, $6)")
            .bind(self.id)
            .bind(&self.title)
            .bind(&self.tags)
            .bind(&self.create_time)
            .bind(&self.update_time)
            .bind(&self.body)
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[get("/blog/article/<name>")]
fn markdown_request(name: String) -> Redirect {
    Redirect::to(format!("/public/markdown/{}.md", name))
}

#[post("/up", format = "json", data = "<blog>")]
pub async fn add_blog(blog: Json<Blog>, pool: &State<Database>) {
    blog.insert(pool.db());
}

#[get("/delet/<id>")]
pub async fn delet_blog_by_id(id: i32, pool: &State<Database>) -> Status {
    sqlx::query("delet from blog where id = $1")
        .bind(id)
        .execute(pool.db())
        .await;
    Status::from_code(200).unwrap()
}

#[post("/update", format = "json", data = "<blog>")]
pub async fn update_blog(blog: Json<Blog>, pool: &State<Database>) {
    sqlx::query(
        "update blog
set title = $1, tags = $2, create_time = $3, update_time = $4, body = $5
where id = $6",
    )
    .bind(&blog.title)
    .bind(&blog.tags)
    .bind(&blog.create_time)
    .bind(&blog.update_time)
    .bind(&blog.body)
    .bind(blog.id)
    .execute(pool.db())
    .await;
}

#[get("/get/<id>")]
pub async fn get_blog_by_id(id: i32, pool: &State<Database>) -> Result<Json<Blog>, Status> {
    let blog = sqlx::query_as::<_, Blog>("select * from blog where id = $1")
        .bind(id)
        .fetch_one(pool.db())
        .await;

    match blog {
        Ok(blog) => Ok(Json::from(blog)),
        _ => Err(Status::NotFound),
    }
}

#[get("/get/list")]
pub async fn get_blog_list(pool: &State<Database>) -> Result<Json<Vec<BlogInfo>>, Status> {
    let blogs = sqlx::query_as::<_, BlogInfo>("select id,title,tags,create_time from blog")
        .fetch_all(pool.db())
        .await;

    match blogs {
        Ok(blogs) => Ok(Json::from(blogs)),
        _ => Err(Status::NotFound),
    }
}
