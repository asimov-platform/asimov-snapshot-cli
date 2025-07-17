// This is free and unencumbered software released into the public domain.

use clientele::{StandardOptions, SysexitsError};

pub fn list(_flags: &StandardOptions) -> Result<(), SysexitsError> {
    let ss = crate::Snapshotter::default();
    let urls = ss.list()?;
    for (url, ts) in urls {
        println!("{url} ({ts}");
    }
    Ok(())
}
