use anyhow::{Ok, Result};
use chrono::{DateTime, NaiveDate, Utc};
use sea_query::{IdenStatic, OnConflict, PostgresQueryBuilder, Query, enum_def};
use sea_query_sqlx::SqlxBinder;
use sqlx::{PgTransaction, Row};
use uuid::Uuid;

use crate::types::InstrumentSource;

#[enum_def]
pub struct SourceFile {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub source: InstrumentSource,
    pub date: NaiveDate,
    pub key: String,
    pub checksum: String,
}

pub struct CreateSourceFileOptions {
    pub source: InstrumentSource,
    pub date: NaiveDate,
    pub key: String,
    pub checksum: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn create_source_file(
    trx: &mut PgTransaction<'_>,
    input: CreateSourceFileOptions,
) -> Result<Uuid> {
    let (sql, values) = Query::insert()
        .into_table(SourceFileIden::Table)
        .columns([
            SourceFileIden::Source,
            SourceFileIden::Date,
            SourceFileIden::Key,
            SourceFileIden::Checksum,
            SourceFileIden::CreatedAt,
            SourceFileIden::UpdatedAt,
        ])
        .values([
            input.source.to_string().into(),
            input.date.into(),
            input.key.into(),
            input.checksum.into(),
            input.created_at.into(),
            input.updated_at.into(),
        ])?
        .on_conflict(
            OnConflict::columns([SourceFileIden::Source, SourceFileIden::Date])
                .update_columns([
                    SourceFileIden::Key,
                    SourceFileIden::Checksum,
                    SourceFileIden::CreatedAt,
                    SourceFileIden::UpdatedAt,
                ])
                .to_owned(),
        )
        .returning_col(SourceFileIden::Id)
        .build_sqlx(PostgresQueryBuilder);

    let row = sqlx::query_with(&sql, values).fetch_one(&mut **trx).await.unwrap();
    let id: Uuid = row.get(SourceFileIden::Id.as_str());

    Ok(id)
}
