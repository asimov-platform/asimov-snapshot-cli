// This is free and unencumbered software released into the public domain.

use asimov_snapshot::{Snapshotter, storage::Fs};
use clientele::{StandardOptions, SysexitsError};
use color_print::{ceprintln, cprintln};
use jiff::{Zoned, tz::TimeZone};

use crate::timestamps::format_ts_diff;

#[tokio::main]
pub async fn log(url: &str, _flags: &StandardOptions) -> Result<(), SysexitsError> {
    let ss = Snapshotter::<Fs>::new_fs()
        .inspect_err(|e| ceprintln!("<s,r>error:</> failed to create snapshotter: {e}"))?;
    let snapshots = ss.log(url).await.inspect_err(|e| {
        ceprintln!("<s,r>error:</> failed to fetch snapshot log for `{url}`: {e}")
    })?;
    let now = Zoned::now();
    for ts in snapshots {
        let data = ss.read(url, ts).await.inspect_err(|e| {
            ceprintln!("<s,r>error:</> failed to read snapshot `{ts}` for url `{url}`: {e}")
        })?;

        let hash = <sha2::Sha256 as sha2::Digest>::digest(&data);
        let hash = &hex::encode(hash)[..8];

        let diff = format_ts_diff(&now, &ts.to_zoned(TimeZone::UTC))
            .expect("Unexpectedly failed to format timestamp difference");

        cprintln!("<s>{hash}</> ({diff:#})");
    }
    Ok(())
}
