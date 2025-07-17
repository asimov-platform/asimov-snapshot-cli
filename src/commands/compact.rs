// This is free and unencumbered software released into the public domain.

use clientele::{StandardOptions, SysexitsError};

pub fn compact(urls: &Vec<String>, _flags: &StandardOptions) -> Result<(), SysexitsError> {
    let ss = crate::Snapshotter::default();
    for url in urls {
        ss.snapshot(url)?;
    }
    Ok(())
}
