// This is free and unencumbered software released into the public domain.

use asimov_env::paths::asimov_root;
use asimov_module::resolve::Resolver;
use asimov_registry::Registry;
use asimov_snapshot::{Options, Snapshotter};
use clientele::{
    StandardOptions,
    SysexitsError::{self, *},
};
use color_print::ceprintln;

#[tokio::main]
pub async fn snapshot(urls: &[String], _flags: &StandardOptions) -> Result<(), SysexitsError> {
    let storage = asimov_snapshot::storage::Fs::for_dir(asimov_root().join("snapshots"))?;
    let modules = Registry::default().enabled_modules().await.map_err(|e| {
        ceprintln!("<s,r>error:</> unable to get enabled modules: {e}");
        EX_UNAVAILABLE
    })?;
    let resolver = Resolver::try_from_iter(modules.into_iter().map(|manifest| manifest.manifest))
        .map_err(|e| {
        ceprintln!("<s,r>error:</> failed to create module resolver: {e}");
        EX_UNAVAILABLE
    })?;
    let mut ss = Snapshotter::new(resolver, storage, Options::default());

    for url in urls {
        ss.snapshot(url).await.inspect_err(|e| {
            ceprintln!("<s,r>error:</> failed to snapshot the resource `{url}`: {e}")
        })?;
    }
    Ok(())
}
