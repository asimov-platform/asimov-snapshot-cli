// This is free and unencumbered software released into the public domain.

use asimov_snapshot::{Snapshotter, storage::Fs};
use clientele::{StandardOptions, SysexitsError};
use color_print::{ceprintln, cprintln};
use jiff::{Zoned, tz::TimeZone};

use crate::timestamps::format_ts_diff;

#[tokio::main]
pub async fn list(_flags: &StandardOptions) -> Result<(), SysexitsError> {
    let ss = Snapshotter::<Fs>::new_fs()
        .inspect_err(|e| ceprintln!("<s,r>error:</> failed to create snapshotter: {e}"))?;
    let now = Zoned::now();
    let urls = ss
        .list()
        .await
        .inspect_err(|e| ceprintln!("<s,r>error:</> failed to list snapshots: {e}"))?;
    for (url, ts) in urls {
        let diff = format_ts_diff(&now, &ts.to_zoned(TimeZone::UTC))
            .expect("Unexpectedly failed to format timestamp difference");

        cprintln!("<s>{url}</> (last updated {diff})");
    }
    Ok(())
}
