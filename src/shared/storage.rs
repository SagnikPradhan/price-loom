use std::string::ToString;
use std::sync::Arc;

use anyhow::{Context, Result};
use bytes::Bytes;
use chrono::NaiveDate;
use object_store::path::Path;
use object_store::{ObjectStore, ObjectStoreExt, PutPayload};
use strum::{Display, IntoStaticStr};
use tracing::{debug, info};
use url::Url;

pub fn get_object_store(url_str: String) -> Result<Arc<dyn ObjectStore>> {
    debug!("Connecting to the object storage");
    let url = Url::parse(&url_str).context("Could not parse object storage URL")?;
    let (store, _) = object_store::parse_url(&url).context("Could not connect to object store")?;
    info!("Connected to the object storage");
    Ok(Arc::from(store))
}

#[derive(Debug)]
pub struct AppFile {
    pub date: NaiveDate,
    pub kind: AppFileKind,
    pub data: Bytes,
}

#[derive(Debug, Display, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum AppFileKind {
    NSE,
}

pub async fn save_file(store: &Arc<dyn ObjectStore>, file: AppFile) -> Result<()> {
    debug!("Saving file - {:?} {:?}", file.kind, file.date);

    let path = get_file_path(&file.date, &file.kind);
    let payload = PutPayload::from_bytes(file.data);
    store.put(&path, payload).await.context(format!("failed to write file to `{}`", path))?;
    info!("Saved file successfully - {:?} {:?}", file.kind, file.date);

    Ok(())
}

pub async fn read_file(
    store: &Arc<dyn ObjectStore>,
    date: &NaiveDate,
    kind: &AppFileKind,
) -> Result<Option<Bytes>> {
    debug!("Reading file - {:?} {:?}", &date, &kind);
    let path = get_file_path(&date, kind);

    match store.get(&path).await {
        Err(object_store::Error::NotFound { .. }) => Ok(None),
        Err(error) => Err(error).with_context(|| format!("failed to read file `{}`", path)),
        Ok(result) => result.bytes().await.map(Some).map_err(anyhow::Error::from),
    }
}

fn get_file_path(date: &NaiveDate, kind: &AppFileKind) -> Path {
    let filename = format!("{}.json", date);
    Path::from(format!("{}/{}", kind.to_string(), filename))
}
