use std::time::SystemTime;
use chrono::prelude::{Local};
use sqlx;
use sqlx::{sqlite::SqliteQueryAs};
use tokio::stream::StreamExt;


#[derive(Debug, sqlx::FromRow)]
pub struct Note{
    id: i32,
    message: String,
    time: i32,
}


pub async fn add_new(msg: String, db: &mut sqlx::SqliteConnection) -> Result<(), sqlx::Error>{
    sqlx::query("INSERT INTO notes
                (Message, Time)
                values ( ? , ? )")
            .bind(msg)
            .bind(Local::now().timestamp())
            .execute(db)
            .await?;
    Ok(())
}

pub async fn get_all(db: &mut sqlx::SqliteConnection) -> Result<Vec<Note>, sqlx::Error>{
    let mut notes_cursor = sqlx::query_as::<_, Note>("SELECT Id, Message, Time FROM notes").fetch(db);
    let mut notes = Vec::new();

    // breaks if there's any error
    while let Some(result) = notes_cursor.next().await {
        notes.push(result.unwrap());
    }
    Ok(notes)
}

pub async fn delete(id: i32, db: &mut sqlx::SqliteConnection) -> Result<(), sqlx::Error>{
    sqlx::query("DELETE FROM notes
                WHERE Id = ?")
            .bind(id)
            .execute(db)
            .await?;
    Ok(())
}
