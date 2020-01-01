// Copyright 2020 Boyd Johnson

//! Entry holds either an Occupied entry or a Vacant entry and
//! allows for methods like `or_insert` and `or_insert_with`.


use crate::store::Store;
use crate::error::StoreError;
use db_key::Key;
use serde::{Serialize, Deserialize};

pub enum Entry<'a, K: Key, T: Serialize + for<'de> Deserialize<'de>> {
    Occupied,
    Vacant(VacantEntry<'a, K, T>),
}

impl<'a, K: Key, T: Serialize + for<'de> Deserialize<'de>> Entry<'a, K, T> {

    pub fn or_insert(self, default: T) -> Result<(), StoreError> {
        if let Entry::Vacant(vacant) = self {
            vacant.insert(default)?;
        }
        Ok(())
    }

    pub fn or_insert_with<F: FnOnce() -> T>(self, f: F) -> Result<(), StoreError> {
        if let Entry::Vacant(vacant) = self {
            vacant.insert_with(f)?;
        }
        Ok(())
    }

    pub fn new_vacant(key: K, store: &'a mut Store<K, T>) -> Self {
        Entry::Vacant(VacantEntry {key: Some(key), store})
    }

}

pub struct VacantEntry<'a, K: Key, T: Serialize + for<'de> Deserialize<'de>> {
    key: Option<K>,
    store: &'a mut Store<K, T>,
}

impl<'a, K: Key, T: Serialize + for<'de> Deserialize<'de>> VacantEntry<'a, K, T> {
    fn insert(self, value: T) -> Result<(), StoreError> {
        if let Some(key) = self.key {
            self.store.insert(key, value)?;
        }
        Ok(())
    }

    fn insert_with<F: FnOnce() -> T>(self, f: F) -> Result<(), StoreError> {
        if let Some(key) = self.key {
            self.store.insert(key, f())?;
        }
        Ok(())
    }
}
