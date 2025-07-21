// This is free and unencumbered software released into the public domain.

use asimov_snapshot::{Snapshotter, storage::Fs};
use clientele::{StandardOptions, SysexitsError};

#[tokio::main]
pub async fn compact(urls: &Vec<String>, _flags: &StandardOptions) -> Result<(), SysexitsError> {
    let ss = Snapshotter::<Fs>::new_fs().expect("Failed to create snapshotter");
    for url in urls {
        ss.compact(url).await?;
    }
    Ok(())
}
