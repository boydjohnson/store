use crate::error::StoreError;
use bincode::{deserialize, serialize};
use db_key::Key;
use leveldb::database::{iterator::Iterable, Database};
use leveldb::kv::KV;
use leveldb::options::{Options, ReadOptions, WriteOptions};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub struct Store<K: Key, T> {
    db: Database<K>,

    phantom: std::marker::PhantomData<T>,
}

impl<K: Key, T> Store<K, T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(path: &Path) -> Result<Store<K, T>, StoreError> {
        let mut options = Options::new();
        options.create_if_missing = true;
        Ok(Store {
            db: Database::open(path, options).map_err(|err| StoreError::DatabaseError(err))?,
            phantom: std::marker::PhantomData::default(),
        })
    }

    pub fn new_store(prefix: &str) -> Result<Store<K, T>, StoreError> {
        let tmp = tempdir::TempDir::new(prefix).map_err(|err| StoreError::FileSystemError(err))?;
        Store::new(tmp.path())
    }

    pub fn insert(&mut self, key: K, value: T) -> Result<(), StoreError> {
        self.db
            .put(
                WriteOptions::new(),
                key,
                &serialize(&value).map_err(|err| StoreError::SerDeError(err))?,
            )
            .map_err(|err| StoreError::DatabaseError(err))
    }

    pub fn get(&self, key: &K) -> Result<Option<T>, StoreError> {
        let result = self.db.get(ReadOptions::new(), key);

        match result {
            Ok(option) => Store::<K, T>::deserialize(option),
            Err(err) => Err(StoreError::DatabaseError(err)),
        }
    }

    pub fn iter_keys<'a>(&'a self) -> impl Iterator<Item = K> + 'a {
        self.db.keys_iter(ReadOptions::new())
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (K, T)> + 'a {
        self.db.iter(ReadOptions::new()).filter_map(|(key, value)| {
            match Store::<K, T>::deserialize_bytes(&value) {
                Ok(val) => Some((key, val)),
                Err(_) => None,
            }
        })
    }

    pub fn iter_values<'a>(&'a self) -> impl Iterator<Item = T> + 'a {
        self.db
            .value_iter(ReadOptions::new())
            .filter_map(|item| Store::<K, T>::deserialize_bytes(&item).ok())
    }

    fn deserialize(option: Option<Vec<u8>>) -> Result<Option<T>, StoreError> {
        let bytes;
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

    fn deserialize_bytes(bytes: &[u8]) -> Result<T, StoreError> {
        deserialize(bytes).map_err(|err| StoreError::SerDeError(err))
    }
}
