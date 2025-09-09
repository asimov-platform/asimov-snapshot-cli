// This is free and unencumbered software released into the public domain.

use asimov_env::paths::asimov_root;
use asimov_module::url::normalize_url;
use asimov_registry::Registry;
use asimov_snapshot::{Options, Snapshotter};
use clientele::{StandardOptions, SysexitsError};

#[tokio::main]
pub async fn compact(urls: &[String], _flags: &StandardOptions) -> Result<(), SysexitsError> {
    let storage = asimov_snapshot::storage::Fs::for_dir(asimov_root().join("snapshots"))?;
    let ss = Snapshotter::new(Registry::default(), storage, Options::default());

    for url in urls {
        let url = normalize_url(url)
            .inspect_err(|e| {
                tracing::error!("proceeding with given unmodified URL, normalization failed: {e}, ")
            })
            .unwrap_or_else(|_| url.into());

        ss.compact(&url)
            .await
            .inspect_err(|e| tracing::error!("failed to compact snapshots for `{url}`: {e}"))?;
    }
    Ok(())
}
