pub mod model;

use std::marker::PhantomData;

use self::model::{Models, User};
use rocket::fairing::{self, Info, Kind};
use rocket::log::private::error;
use rocket::{fairing::Fairing, http::Status};
use rocket::{Build, Rocket, State};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub struct Database(SqlitePool);

impl Database {
    pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new().connect(url).await?;
        Ok(Self(pool))
    }
}

pub struct DbFairing(Option<&'static str>, PhantomData<Database>);

impl DbFairing {
    pub fn new() -> Self {
        Self(None, std::marker::PhantomData)
    }

    pub fn with_name(name: &'static str) -> Self {
        Self(Some(name), std::marker::PhantomData)
    }
}

#[async_trait]
impl Fairing for DbFairing {
    fn info(&self) -> Info {
        Info {
            name: self.0.unwrap_or(std::any::type_name::<Self>()),
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        match Database::new("db/sqlx/db.sqlite").await {
            Ok(db) => Ok(rocket.manage(db)),
            Err(e) => {
                error!("failed to initialize database: {}", e);
                Err(rocket)
            }
        }
    }
}

#[get("/<id>")]
pub async fn hello(pool: &State<Database>, id: i32) -> Result<String, Status> {
    let user = User::find_by(&User::filter_sql("user", "id"), id, &pool.0).await;

    match user {
        Ok(user) => Ok(format!("Hello {}!", &user.name)),
        _ => Err(Status::NotFound),
    }
}
