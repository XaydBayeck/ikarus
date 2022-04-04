pub mod model;

use rocket::{http::Status, State};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

use self::model::User;

pub struct Database(SqlitePool);

impl Database {
    pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new().connect(url).await?;
        Ok(Self(pool))
    }
}

#[get("/<id>")]
pub async fn hello(pool: &State<Database>, id: i32) -> Result<String, Status> {
    let user = User::find_by_id(id, &pool.0).await;

    match user {
        Ok(user) => Ok(format!("Hello {}!", &user.name)),
        _ => Err(Status::NotFound),
    }
}
