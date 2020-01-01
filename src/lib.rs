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



pub mod entry;
pub mod error;
pub mod store;

pub use store::Store;
