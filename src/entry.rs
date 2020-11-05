// Copyright 2020 Boyd Johnson

//! Entry holds either an Occupied entry or a Vacant entry and
//! allows for methods like `or_insert` and `or_insert_with`.

use crate::error::StoreError;
use crate::store::Store;
use db_key::Key;
use serde::{Deserialize, Serialize};

pub enum Entry<'a, K: Key, T: Serialize + for<'de> Deserialize<'de>> {
    Occupied(OccupiedEntry<'a, K, T>),
    Vacant(VacantEntry<'a, K, T>),
}

impl<'a, K: Key + Clone, T: Serialize + for<'de> Deserialize<'de>> Entry<'a, K, T> {
    pub fn or_insert(self, default: T) -> Result<(), StoreError> {
        if let Entry::Vacant(vacant) = self {
            vacant.insert(default)?;
        }
        Ok(())
    }

    pub fn and_modify<F: FnOnce(&mut T)>(mut self, f: F) -> Result<Self, StoreError> {
        if let Entry::Occupied(ref mut entry) = self {
            entry.and_modify(f)?;
        }
        Ok(self)
    }

    pub fn or_insert_with<F: FnOnce() -> T>(self, f: F) -> Result<(), StoreError> {
        if let Entry::Vacant(vacant) = self {
            vacant.insert_with(f)?;
        }
        Ok(())
    }

    pub fn new_vacant(key: K, store: &'a mut Store<K, T>) -> Self {
        Entry::Vacant(VacantEntry { key: key, store })
    }

    pub fn new_occupied(key: K, store: &'a mut Store<K, T>) -> Self {
        Entry::Occupied(OccupiedEntry { key, store })
    }
}

pub struct VacantEntry<'a, K: Key, T: Serialize + for<'de> Deserialize<'de>> {
    key: K,
    store: &'a mut Store<K, T>,
}

impl<'a, K: Key + Clone, T: Serialize + for<'de> Deserialize<'de>> VacantEntry<'a, K, T> {
    fn insert(self, value: T) -> Result<(), StoreError> {
        self.store.insert(self.key.clone(), value)?;

        Ok(())
    }

    fn insert_with<F: FnOnce() -> T>(self, f: F) -> Result<(), StoreError> {
        self.store.insert(self.key.clone(), f())?;

        Ok(())
    }
}

pub struct OccupiedEntry<'a, K: Key, T: Serialize + for<'de> Deserialize<'de>> {
    key: K,
    store: &'a mut Store<K, T>,
}

impl<'a, K: Key + Clone, T: Serialize + for<'de> Deserialize<'de>> OccupiedEntry<'a, K, T> {
    fn and_modify<F: FnOnce(&mut T)>(&mut self, f: F) -> Result<(), StoreError> {
        if let Some(mut val) = self.store.get(&self.key)? {
            f(&mut val);
            self.store.insert(self.key.clone(), val)?;
        }
        Ok(())
    }
}
