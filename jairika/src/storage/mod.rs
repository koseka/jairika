use crate::{Error, Result};

use std::path::PathBuf;
use surrealkv;

pub struct Store {
    entry: surrealkv::Store,
}

impl Store {
    pub fn new(address: &str) -> Result<Self> {
        let mut options = surrealkv::Options::new();
        options.disk_persistence = true;
        options.dir = PathBuf::from(address);
        options.isolation_level = surrealkv::IsolationLevel::SnapshotIsolation; // Or `surrealkv::IsolationLevel::SerializableSnapshotIsolation`
        options.max_key_size = 1024;
        options.max_value_size = 1024 * 1024;
        options.max_value_threshold = 64; // 64 bytes
        options.max_value_cache_size = 100000;
        options.max_segment_size = 1 << 29; // 512 MB
        options.max_compaction_segment_size = 1 << 30; // 1 GB

        match surrealkv::Store::new(options) {
            Ok(store) => Ok(Store { entry: store }),
            Err(e) => Err(Error::ModelCreationFailed(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_surrealkv_store() {
        let mut options = surrealkv::Options::new();
        options.disk_persistence = true;
        options.dir = PathBuf::from("test_surrealkv_store");
        surrealkv::Store::new(options).unwrap();
    }
}
