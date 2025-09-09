// This is free and unencumbered software released into the public domain.

use asimov_env::paths::asimov_root;
use asimov_registry::Registry;
use asimov_snapshot::{Options, Snapshotter};
use clientele::{StandardOptions, SysexitsError};

#[tokio::main]
pub async fn compact(urls: &[String], _flags: &StandardOptions) -> Result<(), SysexitsError> {
    let storage = asimov_snapshot::storage::Fs::for_dir(asimov_root().join("snapshots"))?;
    let ss = Snapshotter::new(Registry::default(), storage, Options::default());

    for url in urls {
        ss.compact(url)
            .await
            .inspect_err(|e| tracing::error!("failed to compact snapshots for `{url}`: {e}"))?;
    }
    Ok(())
}
