// Copyright 2020 Boyd Johnson

//! Errors

use leveldb::database::error::Error;

pub enum StoreError {
    DatabaseError(Error),
    SerDeError(bincode::Error),
    FileSystemError(std::io::Error),
}
