// This is free and unencumbered software released into the public domain.

use asimov_env::paths::asimov_root;
use asimov_module::resolve::Resolver;
use asimov_snapshot::Snapshotter;
use clientele::{StandardOptions, SysexitsError};
use color_print::{ceprintln, cprintln};
use jiff::{Zoned, tz::TimeZone};

use crate::{timestamps::format_ts_diff, url::normalize_url};

#[tokio::main]
pub async fn log(url: &str, _flags: &StandardOptions) -> Result<(), SysexitsError> {
    let storage = asimov_snapshot::storage::Fs::for_dir(asimov_root().join("snapshots"))?;
    let ss = Snapshotter::new(Resolver::new(), storage);

    let mut snapshots = ss.log(url).await.inspect_err(|e| {
        ceprintln!("<s,r>error:</> failed to fetch snapshot log for `{url}`: {e}")
    })?;
    snapshots.sort();

    let now = Zoned::now();
    for ts in snapshots {
        let url = normalize_url(url);
        let data = ss.read(&url, ts).await.inspect_err(|e| {
            ceprintln!("<s,r>error:</> failed to read snapshot `{ts}` for URL `{url}`: {e}")
        })?;

        let hash = <sha2::Sha256 as sha2::Digest>::digest(&data);
        let hash = &hex::encode(hash)[..8];

        let diff = format_ts_diff(&now, &ts.to_zoned(TimeZone::UTC))
            .expect("Unexpectedly failed to format timestamp difference");

        cprintln!("<s>{hash}</> ({diff:#})");
    }
    Ok(())
}
