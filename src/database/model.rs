use sqlx::{FromRow, Result, SqlitePool};

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
}

impl User {
    pub async fn find_by_id(id: i32, pool: &SqlitePool) -> Result<User> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM user WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }
}
