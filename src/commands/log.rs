// This is free and unencumbered software released into the public domain.

use clientele::{StandardOptions, SysexitsError};

pub fn log(url: &str, _flags: &StandardOptions) -> Result<(), SysexitsError> {
    let ss = crate::Snapshotter::new();
    let snapshots = ss.log(url)?;
    for snapshot in snapshots {
        println!("{snapshot}");
    }
    Ok(())
}
