// This is free and unencumbered software released into the public domain.

use asimov_env::paths::asimov_root;
use chrono::{DateTime, prelude::*};
use std::io::Result;

pub mod commands;
pub mod features;
pub mod storage;

pub trait Runner {
    fn fetch(&self, _url: &str) -> Result<Vec<u8>> {
        unimplemented!()
    }
}

pub struct MockRunner;

impl Runner for MockRunner {}

pub struct Snapshotter<S, R> {
    storage: S,
    runner: R,
}

impl Default for Snapshotter<crate::storage::fs::Fs, MockRunner> {
    fn default() -> Self {
        let snapshot_dir = asimov_root().join("snapshots");
        let storage = storage::fs::Fs::for_dir(snapshot_dir).unwrap();
        let runner = MockRunner;
        Self::new(storage, runner)
    }
}

impl<S, R> Snapshotter<S, R> {
    pub fn new(storage: S, runner: R) -> Self {
        Self { storage, runner }
    }
}

impl<S: storage::Storage, R: Runner> Snapshotter<S, R> {
    pub fn snapshot(&self, url: &str) -> Result<()> {
        let timestamp = Utc::now();
        let data = self.runner.fetch(url)?;
        self.storage.save(url, timestamp, data)
    }

    pub fn list(&self) -> Result<Vec<(String, DateTime<Utc>)>> {
        self.storage.list_urls()
    }

    pub fn log(&self, url: &str) -> Result<Vec<DateTime<Utc>>> {
        self.storage.list_snapshots(url)
    }

    #[tracing::instrument(skip(self))]
    pub fn compact(&self, url: &str) -> Result<()> {
        // TODO: max hourly/daily/weekly/monthly/yearly snapshots
        let timestamps = self.storage.list_snapshots(url)?;
        let Some(latest) = timestamps.iter().max() else {
            return Ok(());
        };
        tracing::debug!("Deleting snapshots older than `{latest}`");
        for &ts in timestamps.iter().filter(|&ts| ts != latest) {
            self.storage.delete(url, ts)?;
        }
        Ok(())
    }
}
