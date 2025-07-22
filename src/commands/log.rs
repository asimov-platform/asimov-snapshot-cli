// This is free and unencumbered software released into the public domain.

use asimov_snapshot::{Snapshotter, storage::Fs};
use clientele::{StandardOptions, SysexitsError};
use color_print::{ceprintln, cprintln};
use jiff::{SpanRound, Timestamp, Unit};

#[tokio::main]
pub async fn log(url: &str, _flags: &StandardOptions) -> Result<(), SysexitsError> {
    let ss = Snapshotter::<Fs>::new_fs().expect("Failed to create snapshotter");
    let snapshots = ss.log(url).await.inspect_err(|e| {
        ceprintln!("<s,r>error:</> failed to fetch snapshot log for `{url}`: {e}")
    })?;
    let now = Timestamp::now();
    for ts in snapshots {
        let data = ss.read(url, ts).await.inspect_err(|e| {
            ceprintln!("<s,r>error:</> failed to read snapshot `{ts}` for url `{url}`: {e}")
        })?;

        let hash = <sha2::Sha256 as sha2::Digest>::digest(&data);
        let hash = &hex::encode(hash)[..8];

        let diff = now - ts;

        // TODO: largest/smallest rounding units based on the difference
        // e.g.: if diff >= 1.years() => (1.year(), 1.week())
        //       if diff >= 1.weeks() => (1.week(), 1.day())
        //       ...

        let since = diff
            .round(
                SpanRound::new()
                    .days_are_24_hours()
                    .largest(Unit::Day)
                    .smallest(Unit::Second),
            )
            .unwrap();

        cprintln!("<s>{hash}</> ({since:#} ago)");
    }
    Ok(())
}
