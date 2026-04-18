use anyhow::{Ok, Result};
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::PgTransaction;
use uuid::Uuid;

use crate::types::InstrumentSource;

pub struct CreateSourceFile {
    pub source: InstrumentSource,
    pub date: NaiveDate,
    pub key: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn create_source_file(
    trx: &mut PgTransaction<'_>,
    input: CreateSourceFile,
) -> Result<Uuid> {
    let row = sqlx::query!(
        "
            INSERT INTO source_file (
                key,
                date,
                source,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (source, date)
            DO UPDATE SET
                key = EXCLUDED.key,
                updated_at = EXCLUDED.updated_at
            RETURNING id
        ",
        input.key,
        input.date,
        input.source as _,
        input.created_at,
        input.updated_at
    )
    .fetch_one(&mut **trx)
    .await
    .unwrap();

    Ok(row.id)
}
