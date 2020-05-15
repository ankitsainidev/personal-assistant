use sqlx::{self, sqlite::SqliteQueryAs};
use std::fmt;
use tokio::stream::StreamExt;

#[derive(Debug, sqlx::FromRow)]
pub struct Save {
    pub key: String,
    pub val: String,
}

impl fmt::Display for Save {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{key:<10} - {val}", key = self.key, val = self.val)
    }
}

pub async fn add_new(
    key: String,
    value: String,
    db: &mut sqlx::SqliteConnection,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO saves
                (key, val)
                values ( ? , ? )",
    )
    .bind(key)
    .bind(value)
    .execute(db)
    .await?;
    Ok(())
}

pub async fn get_all(db: &mut sqlx::SqliteConnection) -> Result<Vec<Save>, sqlx::Error> {
    let mut saves_curosr = sqlx::query_as::<_, Save>("SELECT key, val FROM saves").fetch(db);
    let mut saves = Vec::new();

    // breaks if there's any error
    while let Some(result) = saves_curosr.next().await {
        saves.push(result.unwrap());
    }
    Ok(saves)
}

pub async fn does_exists(key: String, db: &mut sqlx::SqliteConnection) -> bool {
    match get(key, db).await {
        Ok(_save) => true,
        Err(_e) => false,
    }
}

pub async fn get(key: String, db: &mut sqlx::SqliteConnection) -> Result<Save, sqlx::Error> {
    Ok(
        sqlx::query_as::<_, Save>("SELECT key, val FROM saves WHERE key=?")
            .bind(key)
            .fetch_one(db)
            .await?,
    )
}

pub async fn delete(key: String, db: &mut sqlx::SqliteConnection) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM saves WHERE key=?")
        .bind(key)
        .execute(db)
        .await?;
    Ok(())
}
