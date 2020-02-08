#[cfg(feature = "long-key-impls")]
use arrayref::array_ref;
use db_key::Key;
#[cfg(feature = "long-key-impls")]
use std::array::FixedSizeArray;

#[cfg(feature = "long-key-impls")]
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Long(pub i64);

#[cfg(feature = "long-key-impls")]
impl Key for Long {
    fn from_u8(other: &[u8]) -> Self {
        Long::from(other)
    }

    fn as_slice<T, F: Fn(&[u8]) -> T>(&self, f: F) -> T {
        f(self.0.to_be_bytes().as_slice())
    }
}

#[cfg(feature = "long-key-impls")]
impl From<&[u8]> for Long {
    fn from(other: &[u8]) -> Self {
        let inner = i64::from_be_bytes(array_ref!(other, 0, 8).clone());
        Long(inner)
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct StringW(pub String);

impl Key for StringW {
    fn from_u8(other: &[u8]) -> Self {
        StringW(
            std::string::String::from_utf8(other.to_vec())
                .expect("Non UTF-8 bytes found in the key"),
        )
    }

    fn as_slice<T, F: Fn(&[u8]) -> T>(&self, f: F) -> T {
        f(self.0.as_bytes())
    }
}
