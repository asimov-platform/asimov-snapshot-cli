// This is free and unencumbered software released into the public domain.

use asimov_snapshot::{Snapshotter, storage::Fs};
use clientele::{StandardOptions, SysexitsError};
use color_print::{ceprintln, cprintln};
use jiff::{SpanRound, Timestamp, Unit};

#[tokio::main]
pub async fn list(_flags: &StandardOptions) -> Result<(), SysexitsError> {
    let ss = Snapshotter::<Fs>::new_fs()
        .inspect_err(|e| ceprintln!("<s,r>error:</> Failed to create snapshotter: {e}"))?;
    let now = Timestamp::now();
    let urls = ss
        .list()
        .await
        .inspect_err(|e| ceprintln!("<s,r>error:</> failed to list snapshots: {e}"))?;
    for (url, ts) in urls {
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

        cprintln!("<s>{url}</> (last updated {since:#} ago)");
    }
    Ok(())
}
