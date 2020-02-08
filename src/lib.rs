// Copyright 2020 Boyd Johnson

//! [Store](struct.Store.html) provides serialization over `leveldb` with `bincode`.
//! This crate may be useful for creating cli tools where items must be stored and
//! memory is a concern.
//!
//! # Compilation
//! Install the OS specific dependencies for [leveldb](https://crates.io/crates/leveldb).
//!
//! # Usage
//! impl `db_key::Key` for the Key
//! impl `serde::Serialize` and `for<'de> serde::Deserialize<'de>` for the Value

#![cfg_attr(feature = "long-key-impls", feature(fixed_size_array))]

pub mod entry;
pub mod error;
pub mod key_impl;
pub mod store;

pub use crate::store::Store;
