// This is free and unencumbered software released into the public domain.

use asimov_env::paths::asimov_root;
use asimov_module::resolve::Resolver;
use asimov_snapshot::Snapshotter;
use clientele::{StandardOptions, SysexitsError};
use color_print::{ceprintln, cprintln};
use jiff::{Zoned, tz::TimeZone};

use crate::timestamps::format_ts_diff;

#[tokio::main]
pub async fn list(_flags: &StandardOptions) -> Result<(), SysexitsError> {
    let storage = asimov_snapshot::storage::Fs::for_dir(asimov_root().join("snapshots"))?;
    let ss = Snapshotter::new(Resolver::new(), storage);

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
