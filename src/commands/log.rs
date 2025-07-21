// This is free and unencumbered software released into the public domain.

use asimov_snapshot::{Snapshotter, storage::Fs};
use clientele::{StandardOptions, SysexitsError};
use color_print::ceprintln;

#[tokio::main]
pub async fn log(url: &str, _flags: &StandardOptions) -> Result<(), SysexitsError> {
    let ss = Snapshotter::<Fs>::new_fs().expect("Failed to create snapshotter");
    let snapshots = ss.log(url).await.inspect_err(|e| {
        ceprintln!("<s,r>error:</> failed to fetch snapshot log for `{url}`: {e}")
    })?;
    for snapshot in snapshots {
        println!("{snapshot}");
    }
    Ok(())
}
