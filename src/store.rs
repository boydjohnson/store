use crate::error::StoreError;
use bincode::{deserialize, serialize};
use db_key::Key;
use leveldb::database::Database;
use leveldb::kv::KV;
use leveldb::options::{Options, ReadOptions, WriteOptions};
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::path::Path;

pub struct Store<K: Key + Eq + Hash> {
    db: Database<K>,
}

impl<K: Key + Eq + Hash> Store<K> {
    pub fn new(path: &Path) -> Result<Store<K>, StoreError> {
        let mut options = Options::new();
        options.create_if_missing = true;
        Ok(Store {
            db: Database::open(path, options).map_err(|err| StoreError::DatabaseError(err))?,
        })
    }

    pub fn insert<T>(&mut self, key: K, value: T) -> Result<(), StoreError>
    where
        T: Serialize,
    {
        self.db
            .put(
                WriteOptions::new(),
                key,
                &serialize(&value).map_err(|err| StoreError::SerDeError(err))?,
            )
            .map_err(|err| StoreError::DatabaseError(err))
    }

    pub fn get<T>(&self, key: &K) -> Result<Option<T>, StoreError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let bytes;
        match self.db.get(ReadOptions::new(), key) {
            Ok(option) => {
                if let Some(value_bytes) = option {
                    bytes = value_bytes;
                    match deserialize(&bytes) {
                        Err(err) => Err(StoreError::SerDeError(err)),
                        Ok(item) => Ok(Some(item)),
                    }
                } else {
                    Ok(None)
                }
            }
            Err(err) => Err(StoreError::DatabaseError(err)),
        }
    }
}
