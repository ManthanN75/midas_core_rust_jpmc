use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::env;

pub async fn connect() -> Result<Pool<Sqlite>, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&db_url).await?;
    Ok(pool)
}
