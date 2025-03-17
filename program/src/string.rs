#[cfg(any(kani, feature = "kani"))]
use crate::vec::fast::{Vec, VecIterator};
#[cfg(not(any(kani, feature = "kani")))]
use std::vec::Vec;

use std::str::Chars;

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, Eq, PartialEq)]

pub struct String {
    vec: Vec<u8>,
}

impl String {
    pub fn new() -> String {
        String { vec: Vec::new() }
    }

    pub fn chars(&self) -> Chars {
        Box::leak(Box::new(std::string::String::from_utf8(self.vec.as_ref().to_vec()).unwrap())).chars()
    }
}

impl Default for String {
    fn default() -> Self {
        String::new()
    }
}

#[cfg(any(kani, feature = "kani"))]
impl kani::Arbitrary for String {
    fn any() -> Self {
        Self { vec: kani::any() }
    }
}
