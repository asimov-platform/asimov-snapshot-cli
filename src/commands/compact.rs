// This is free and unencumbered software released into the public domain.

use asimov_snapshot::{Snapshotter, storage::Fs};
use clientele::{StandardOptions, SysexitsError};
use color_print::ceprintln;

#[tokio::main]
pub async fn compact(urls: &[String], _flags: &StandardOptions) -> Result<(), SysexitsError> {
    let ss = Snapshotter::<Fs>::new_fs()
        .inspect_err(|e| ceprintln!("<s,r>error:</> failed to create snapshotter: {e}"))?;
    for url in urls {
        ss.compact(url).await.inspect_err(|e| {
            ceprintln!("<s,r>error:</> failed to compact snapshots for `{url}`: {e}")
        })?;
    }
    Ok(())
}
