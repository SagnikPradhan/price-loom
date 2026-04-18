use std::string::ToString;
use std::sync::Arc;

use anyhow::{Context, Result};
use bytes::Bytes;
use chrono::NaiveDate;
use object_store::path::Path;
use object_store::{ObjectStore, ObjectStoreExt, PutPayload};
use tracing::{debug, info};
use url::Url;

use crate::types::InstrumentSource;

pub fn get_store(url_str: String) -> Result<Arc<dyn ObjectStore>> {
    debug!("Connecting to the object storage");
    let url = Url::parse(&url_str).context("Could not parse object storage URL")?;
    let (store, _) = object_store::parse_url(&url).context("Could not connect to object store")?;
    info!("Connected to the object storage");
    Ok(Arc::from(store))
}

#[derive(Debug)]
pub struct SaveSourceFileOptions {
    pub date: NaiveDate,
    pub source: InstrumentSource,
    pub data: Bytes,
}

pub async fn save_file(store: &Arc<dyn ObjectStore>, file: SaveSourceFileOptions) -> Result<Path> {
    debug!("Saving file - {:?} {:?}", file.source, file.date);

    let path = get_file_path(&file.date, &file.source);
    let payload = PutPayload::from_bytes(file.data);
    store.put(&path, payload).await.context(format!("failed to write file to `{}`", path))?;
    info!("Saved file successfully - {:?} {:?}", file.source, file.date);

    Ok(path)
}

pub async fn read_file(
    store: &Arc<dyn ObjectStore>,
    date: &NaiveDate,
    source: &InstrumentSource,
) -> Result<Option<(Path, Bytes)>> {
    debug!("Reading file - {:?} {:?}", date, source);
    let path = get_file_path(date, source);

    match store.get(&path).await {
        Err(object_store::Error::NotFound { .. }) => Ok(None),
        Err(error) => Err(error).with_context(|| format!("failed to read file `{}`", path)),
        Ok(data) => data.bytes().await.map(|f| Some((path, f))).map_err(anyhow::Error::from),
    }
}

fn get_file_path(date: &NaiveDate, kind: &InstrumentSource) -> Path {
    let filename = format!("{}.json", date);
    Path::from(format!("{}/{}", kind, filename))
}
