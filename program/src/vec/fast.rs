use core::slice;
use std::ops;

use hex::FromHex;

use crate::error::Error;
use borsh::{BorshDeserialize, BorshSerialize};

const VEC_SIZE: usize = 10;

#[derive(Clone, Copy, Debug, BorshSerialize, BorshDeserialize, Eq, PartialEq)]
pub struct Vec<T> {
    pub data: [T; VEC_SIZE],
    pub size: usize,
}

#[derive(Debug)]
pub struct VecIterator<'a, T> {
    vec: &'a Vec<T>,
    idx: usize,
}

pub struct VecIntoIterator<T> {
    vec: Vec<T>,
    idx: usize,
}

impl<T: Default> Default for Vec<T> {
    fn default() -> Self {
        Vec {
            data: Default::default(),
            size: 0,
        }
    }
}

impl<T> Vec<T> {
    pub fn new() -> Vec<T>
    where
        T: Default,
    {
        Default::default()
    }

    pub fn with_capacity(_s: usize) -> Vec<T>
    where
        T: Default + Copy,
    {
        Vec::new()
    }

    pub fn push(&mut self, t: T) {
        self.data[self.size] = t;
        self.size += 1;
    }

    pub fn insert(&mut self, pos: usize, t: T) {
        if pos >= self.size {
            panic!("oob");
        }

        let mut v = std::mem::replace(&mut self.data[pos], t);
        for i in pos + 1..self.size + 1 {
            v = std::mem::replace(&mut self.data[i], v);
        }
        self.size += 1;
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn iter(&self) -> VecIterator<T> {
        VecIterator { vec: self, idx: 0 }
    }

    pub fn contains(&self, t: &T) -> bool
    where
        T: PartialEq,
    {
        for i in 0..self.size {
            if &self.data[i] == t {
                return true;
            }
        }

        false
    }

    pub fn sort(&mut self) {
        // nothing
    }

    pub fn dedup(&mut self) {
        // todo
    }

    pub fn remove(&mut self, idx: usize)
    where
        T: Copy,
    {
        if idx >= self.size {
            panic!("oob");
        }

        for i in idx..self.size - 1 {
            self.data[i] = self.data[i + 1];
        }
        self.size -= 1;
    }

    pub fn binary_search(&self, t: &T) -> std::result::Result<usize, usize>
    where
        T: PartialEq,
    {
        for i in 0..self.size {
            if &self.data[i] == t {
                return Ok(i);
            }
        }

        Err(self.size)
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data[..self.size]
    }

    pub fn extend_from_slice(&mut self, slice: &[T])
    where
        T: Copy,
    {
        for z in slice {
            self.push(*z);
        }
    }
}

impl<T> ops::Deref for Vec<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.data.as_ptr(), self.size) }
    }
}

impl<T> ops::DerefMut for Vec<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.data.as_mut_ptr(), self.size) }
    }
}

impl<'a, T> Iterator for VecIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.vec.size {
            return None;
        }

        let res = &self.vec.data[self.idx];
        self.idx += 1;
        Some(res)
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = VecIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Clone> Iterator for VecIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.vec.size {
            return None;
        }

        let res = self.vec.data[self.idx].clone();
        self.idx += 1;
        Some(res)
    }
}

impl<T: Clone> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = VecIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        VecIntoIterator { vec: self, idx: 0 }
    }
}

impl<T: Default> FromIterator<T> for Vec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut v = Vec::new();
        for x in iter {
            v.push(x);
        }
        v
    }
}

impl<T: Default> FromIterator<std::vec::Vec<T>> for Vec<Vec<T>> {
    fn from_iter<I: IntoIterator<Item = std::vec::Vec<T>>>(iter: I) -> Self {
        let mut v = Vec::new();
        for x in iter {
            v.push(x.into());
        }
        v
    }
}

impl<T: Default, const N: usize> From<[T; N]> for Vec<T> {
    fn from(arr: [T; N]) -> Vec<T> {
        let mut vec = Vec::new();
        for element in arr {
            vec.push(element);
        }
        vec
    }
}

impl<T: Default> From<std::vec::Vec<T>> for Vec<T> {
    fn from(value: std::vec::Vec<T>) -> Self {
        let mut res = Vec::new();
        for v in value.into_iter() {
            res.push(v);
        }
        res
    }
}

impl FromHex for Vec<u8> {
    type Error = Error;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> std::result::Result<Self, Self::Error> {
        fn val(c: u8) -> std::result::Result<u8, Error> {
            match c {
                b'A'..=b'F' => Ok(c - b'A' + 10),
                b'a'..=b'f' => Ok(c - b'a' + 10),
                b'0'..=b'9' => Ok(c - b'0'),
                _ => Err(Error::Generic),
            }
        }

        let hex = hex.as_ref();
        if hex.len() % 2 != 0 {
            return Err(Error::Generic);
        }

        hex.chunks(2)
            .map(|pair| Ok(val(pair[0])? << 4 | val(pair[1])?))
            .collect()
    }
}

#[cfg(any(kani, feature = "kani"))]
impl<T: kani::Arbitrary + Default> kani::Arbitrary for Vec<T> {
    fn any() -> Self {
        let mut v = Vec::new();
        for _ in 0..kani::any::<u8>() % (VEC_SIZE as u8) {
            v.push(kani::any());
        }
        v
    }
}
