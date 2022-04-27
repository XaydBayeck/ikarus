use chrono::{DateTime, Utc};
use rocket::{
    http::Status,
    response::{status, Redirect},
    serde::json::Json,
    State,
};
use serde::{Deserialize, Serialize};
pub use sqlx::{Decode, Encode, FromRow, Sqlite, Type};

use crate::database::Database;

type Strings = sqlx::types::Json<Vec<String>>;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Blog {
    pub id: i32,
    pub title: String,
    pub tags: Option<Strings>,
    pub create_time: DateTime<Utc>,
    pub update_time: Option<DateTime<Utc>>,
    pub body: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct BlogInfo {
    pub id: i32,
    pub title: String,
    pub tags: Option<Strings>,
    pub create_time: DateTime<Utc>,
}

#[get("/blog/article/<name>")]
fn markdown_request(name: String) -> Redirect {
    Redirect::to(format!("/public/markdown/{}.md", name))
}

#[post("/up", format = "json", data = "<blog>")]
pub async fn add_blog(blog: Json<Blog>, pool: &State<Database>) -> Status {
    match sqlx::query("insert into blog values ($1, $2, $3, $4, $5, $6)")
        .bind(blog.id)
        .bind(&blog.title)
        .bind(&blog.tags)
        .bind(&blog.create_time)
        .bind(&blog.update_time)
        .bind(&blog.body)
        .execute(pool.db())
        .await
    {
        Ok(_) => Status::Created,
        Err(_) => Status::Conflict,
    }
}

#[get("/delet/<id>")]
pub async fn delet_blog_by_id(
    id: i32,
    pool: &State<Database>,
) -> Result<status::Accepted<()>, status::NotFound<String>> {
    match sqlx::query("delet from blog where id = $1")
        .bind(id)
        .execute(pool.db())
        .await
    {
        Ok(_) => Ok(status::Accepted::<()>(None)),
        Err(_) => Err(status::NotFound(
            "Not find object you want to delet!".to_string(),
        )),
    }
}

#[post("/update", format = "json", data = "<blog>")]
pub async fn update_blog(
    blog: Json<Blog>,
    pool: &State<Database>,
) -> Result<status::Accepted<()>, status::NotFound<String>> {
    match sqlx::query(
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
    .await
    {
        Ok(_) => Ok(status::Accepted::<()>(None)),
        Err(_) => Err(status::NotFound(
            "May be you should add it before.".to_string(),
        )),
    }
}

#[get("/get/select?<id>&<title>")]
pub async fn get_blog_by_id(
    id: Option<i32>,
    title: Option<String>,
    pool: &State<Database>,
) -> Result<Json<Blog>, Status> {
    let blog = match (id, title) {
        (None, None) => Err(sqlx::error::Error::RowNotFound),
        (None, Some(title)) => {
            sqlx::query_as::<_, Blog>("select * from blog where title = $1")
                .bind(title)
                .fetch_one(pool.db())
                .await
        }
        (Some(id), None) => {
            sqlx::query_as::<_, Blog>("select * from blog where id = $1")
                .bind(id)
                .fetch_one(pool.db())
                .await
        }
        (Some(id), Some(title)) => {
            sqlx::query_as::<_, Blog>("select * from blog where id = $1 and title = $2")
                .bind(id)
                .bind(title)
                .fetch_one(pool.db())
                .await
        }
    };

    match blog {
        Ok(blog) => Ok(Json::from(blog)),
        _ => Err(Status::NotFound),
    }
}

#[get("/get/list?<tag>")]
pub async fn get_blog_list(
    tag: Option<String>,
    pool: &State<Database>,
) -> Result<Json<Vec<BlogInfo>>, Status> {
    let blogs = if let Some(tag) = tag {
        sqlx::query_as::<_, BlogInfo>(
            "select id, title, tags, create_time from blog where tags glob $1",
        )
        .bind(format!("*\"{}*\"", tag))
        .fetch_all(pool.db())
        .await
    } else {
        sqlx::query_as::<_, BlogInfo>("select id,title,tags,create_time from blog")
            .fetch_all(pool.db())
            .await
    };

    match blogs {
        Ok(blogs) => Ok(Json::from(blogs)),
        _ => Err(Status::NotFound),
    }
}
