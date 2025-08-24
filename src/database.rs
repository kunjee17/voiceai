use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use anyhow::Result;
use crate::models;

pub async fn init_db(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS recordings (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            audio_data BLOB NOT NULL,
            transcription TEXT,
            summary TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn save_recording(
    pool: &SqlitePool,
    id: &str,
    title: &str,
    audio_data: &[u8],
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO recordings (id, title, audio_data)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(id)
    .bind(title)
    .bind(audio_data)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_transcription(
    pool: &SqlitePool,
    id: &str,
    transcription: &str,
) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE recordings 
        SET transcription = ?, updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
        "#,
    )
    .bind(transcription)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_summary(
    pool: &SqlitePool,
    id: &str,
    summary: &str,
) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE recordings 
        SET summary = ?, updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
        "#,
    )
    .bind(summary)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_all_recordings(pool: &SqlitePool) -> Result<Vec<models::RecordingListItem>> {
    let rows = sqlx::query(
        r#"
        SELECT id, title, transcription, summary, created_at, updated_at
        FROM recordings
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut recordings = Vec::new();
    for row in rows {
        recordings.push(models::RecordingListItem {
            id: row.get("id"),
            title: row.get("title"),
            transcription: row.get("transcription"),
            summary: row.get("summary"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        });
    }

    Ok(recordings)
}

pub async fn get_recording(pool: &SqlitePool, id: &str) -> Result<Option<models::Recording>> {
    let row = sqlx::query(
        r#"
        SELECT id, title, audio_data, transcription, summary, created_at, updated_at
        FROM recordings
        WHERE id = ?
        "#
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    if let Some(row) = row {
        Ok(Some(models::Recording {
            id: row.get("id"),
            title: row.get("title"),
            audio_data: row.get("audio_data"),
            transcription: row.get("transcription"),
            summary: row.get("summary"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }))
    } else {
        Ok(None)
    }
}