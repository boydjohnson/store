// Copyright 2020 Boyd Johnson


//! Store is a serialization wrapper around leveldb.

use crate::error::StoreError;
use bincode::{deserialize, serialize};
use db_key::Key;
use leveldb::database::{iterator::Iterable, Database};
use leveldb::kv::KV;
use leveldb::options::{Options, ReadOptions, WriteOptions};
use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::entry::Entry;

/// A Key value store over LevelDb that adds serialization/deserialization with `bincode`
pub struct Store<K: Key, T> {
    db: Database<K>,

    phantom: std::marker::PhantomData<T>,
}

impl<K: Key, T> Store<K, T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    /// `new` makes a new Store from a Path.
    pub fn new(path: &Path) -> Result<Store<K, T>, StoreError> {
        let mut options = Options::new();
        options.create_if_missing = true;
        Ok(Store {
            db: Database::open(path, options).map_err(|err| StoreError::DatabaseError(err))?,
            phantom: std::marker::PhantomData::default(),
        })
    }

    /// `tmp_store` creates a Store in the tmp directory with a prefix.
    pub fn tmp_store(prefix: &str) -> Result<Store<K, T>, StoreError> {
        let tmp = tempdir::TempDir::new(prefix).map_err(|err| StoreError::FileSystemError(err))?;
        Store::new(tmp.path())
    }

    /// `insert` allows inserting a key and a value.
    pub fn insert(&mut self, key: K, value: T) -> Result<(), StoreError> {
        self.db
            .put(
                WriteOptions::new(),
                key,
                &serialize(&value).map_err(|err| StoreError::SerDeError(err))?,
            )
            .map_err(|err| StoreError::DatabaseError(err))
    }

    /// `get` retrieves the value associated with a key.
    pub fn get(&self, key: &K) -> Result<Option<T>, StoreError> {
        let result = self.db.get(ReadOptions::new(), key);

        match result {
            Ok(option) => Store::<K, T>::deserialize(option),
            Err(err) => Err(StoreError::DatabaseError(err)),
        }
    }

    /// `entry` returns an Entry.
    /// 
    /// # Example
    /// 
    /// .entry(key).or_insert(value)
    /// 
    /// # See
    /// 
    /// [Entry](../entry/enum.Entry.html)
    /// 
    pub fn entry<'a>(&'a mut self, key: K) -> Result<Entry<'a, K, T>, StoreError> {
        match self.get(&key)? {
            Some(_) => Ok(Entry::Occupied),
            None => Ok(Entry::new_vacant(key, self)),
        }
    }

    /// An iterator over the keys in Store.
    pub fn iter_keys<'a>(&'a self) -> impl Iterator<Item = K> + 'a {
        self.db.keys_iter(ReadOptions::new())
    }

    /// An iterator over the keys and values in Store.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (K, T)> + 'a {
        self.db.iter(ReadOptions::new()).filter_map(|(key, value)| {
            match Store::<K, T>::deserialize_bytes(&value) {
                Ok(val) => Some((key, val)),
                Err(_) => None,
            }
        })
    }

    /// An iterator over the values in Store.
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
