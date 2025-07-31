// This is free and unencumbered software released into the public domain.

use asimov_env::paths::asimov_root;
use asimov_module::resolve::Resolver;
use asimov_snapshot::Snapshotter;
use clientele::{StandardOptions, SysexitsError};
use color_print::ceprintln;

#[tokio::main]
pub async fn compact(urls: &[String], _flags: &StandardOptions) -> Result<(), SysexitsError> {
    let storage = asimov_snapshot::storage::Fs::for_dir(asimov_root().join("snapshots"))?;
    let ss = Snapshotter::new(Resolver::new(), storage);

    let urls: Vec<String> = if !urls.is_empty() {
        urls.into()
    } else {
        ss.list()
            .await
            .inspect_err(|e| ceprintln!("<s,r>error:</> failed to list URLs: {e}"))?
            .into_iter()
            .map(|(url, _)| url)
            .collect()
    };

    for url in urls {
        ss.compact(&url).await.inspect_err(|e| {
            ceprintln!("<s,r>error:</> failed to compact snapshots for `{url}`: {e}")
        })?;
    }
    Ok(())
}
