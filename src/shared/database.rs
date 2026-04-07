use anyhow::{Ok, Result};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tracing::{debug, info};

pub async fn get_db_connection(connection_uri: &String) -> Result<Pool<Postgres>> {
    debug!("Connecting to database");
    let pool = PgPoolOptions::new().max_connections(10).connect(connection_uri).await?;
    info!("Connected to the database");
    Ok(pool)
}
