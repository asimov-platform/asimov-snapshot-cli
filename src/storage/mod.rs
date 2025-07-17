// This is free and unencumbered software released into the public domain.

pub mod fs;

use chrono::{DateTime, prelude::*};
use std::io::Result;

pub trait Storage {
    fn save(
        &self,
        url: impl AsRef<str>,
        timestamp: DateTime<chrono::Utc>,
        data: impl AsRef<[u8]>,
    ) -> Result<()> {
        self.save_timestamp(&url, timestamp, data)?;
        match self.current_version(&url) {
            Ok(current) if timestamp > current => self.set_current_version(&url, timestamp),
            Ok(_) => Ok(()),

            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                self.set_current_version(&url, timestamp)
            },
            Err(err) => Err(err),
        }
    }

    fn save_timestamp(
        &self,
        _url: impl AsRef<str>,
        _timestamp: DateTime<Utc>,
        _data: impl AsRef<[u8]>,
    ) -> Result<()> {
        unimplemented!()
    }

    fn read(&self, _url: impl AsRef<str>, _timestamp: DateTime<Utc>) -> Result<Vec<u8>> {
        unimplemented!()
    }

    fn read_current(&self, url: impl AsRef<str>) -> Result<Vec<u8>> {
        let ts = self.current_version(&url)?;
        self.read(&url, ts)
    }

    fn set_current_version(&self, _url: impl AsRef<str>, _timestamp: DateTime<Utc>) -> Result<()> {
        unimplemented!()
    }

    fn current_version(&self, _url: impl AsRef<str>) -> Result<DateTime<Utc>> {
        unimplemented!()
    }

    fn list_urls(&self) -> Result<Vec<(String, DateTime<Utc>)>> {
        unimplemented!()
    }

    fn list_snapshots(&self, _url: impl AsRef<str>) -> Result<Vec<DateTime<Utc>>> {
        unimplemented!()
    }

    fn delete(&self, _url: impl AsRef<str>, _timestamp: DateTime<Utc>) -> Result<()> {
        unimplemented!()
    }
}
