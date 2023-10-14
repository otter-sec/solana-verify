#[cfg(any(kani, feature = "kani"))]
use crate::vec::fast::Vec;
#[cfg(not(any(kani, feature = "kani")))]
use std::vec::Vec;

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, Eq, PartialEq)]

pub struct String {
    vec: Vec<u8>,
}

impl String {
    pub fn new() -> String {
        String { vec: Vec::new() }
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
