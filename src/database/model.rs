use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, Encode, FromRow, Result, Sqlite, SqlitePool, Type};

#[async_trait]
pub trait Models
where
    Self: Sized + for<'r> FromRow<'r, SqliteRow> + Send + Unpin,
{
    async fn find_by_id(id: i32, pool: &SqlitePool) -> Result<Self> {
        let obj = sqlx::query_as::<_, Self>("SELECT * FROM user WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(obj)
    }

    fn filter_sql(table: &str, attr: &str) -> String {
        format!("select * from {} where {} = $1", table, attr)
    }

    async fn find_by<'q, T>(sql: &'q str, item: T, pool: &SqlitePool) -> Result<Self>
    where
        T: 'q + Send + Encode<'q, Sqlite> + Type<Sqlite>,
    {
        let obj = sqlx::query_as::<_, Self>(&sql)
            .bind(item)
            .fetch_one(pool)
            .await?;

        Ok(obj)
    }
}

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[async_trait]
impl Models for User {}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Blog {
    pub id: i32,
    pub title: String,
    pub tags: Option<String>,
    pub create_time: String,
    pub update_time: Option<String>,
    pub body: String,
}

#[async_trait]
impl Models for Blog {}

impl Blog {
    pub async fn insert(&self, pool: &SqlitePool) -> Result<()> {
        sqlx::query("insert into blog value ($1, $2, $3, $4, $5, $6)")
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
