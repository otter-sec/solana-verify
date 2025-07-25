use core::slice;
use std::{
    mem::{ManuallyDrop, MaybeUninit},
    ops,
};

use hex::FromHex;

use crate::error::Error;
use borsh::{BorshDeserialize, BorshSerialize};

const VEC_SIZE: usize = 10;

// FIXME: MaybeUninit means values won't get dropped, but this might be good for solve performance
#[derive(Debug)]
pub struct Vec<T> {
    pub data: [MaybeUninit<T>; VEC_SIZE],
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
            data: [const { MaybeUninit::uninit() }; VEC_SIZE],
            size: 0,
        }
    }
}
impl<T: Default> Vec<T> {
    pub fn append(&mut self, other: &mut Self) {
        for a in &mut other[..] {
            self.push(std::mem::take(a));
        }
    }
}

impl<T: PartialEq> PartialEq for Vec<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: BorshSerialize> BorshSerialize for Vec<T> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.as_slice().serialize(writer)
    }
}

impl<T: BorshDeserialize> BorshDeserialize for Vec<T> {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        kani::assert(false, "deserialization not implemented");
        unreachable!()
    }
}

impl<T: Eq> Eq for Vec<T> {}

impl<T: Copy> Copy for Vec<T> {}

impl<T: Clone> Clone for Vec<T> {
    fn clone(&self) -> Self {
        let mut data = [const { MaybeUninit::uninit() }; VEC_SIZE];
        let mut i = 0;
        while i < VEC_SIZE {
            kani::assume(i < VEC_SIZE);
            if i < self.len() {
                data[i] = unsafe { MaybeUninit::new(self.get_unchecked(i).clone()) };
            }
            i += 1;
        }

        Self {
            data,
            size: self.size,
        }
    }
}

impl<T> Vec<T> {
    pub fn new() -> Vec<T> {
        Vec {
            data: [const { MaybeUninit::uninit() }; VEC_SIZE],
            size: 0,
        }
    }

    pub fn with_capacity(_s: usize) -> Vec<T> {
        Vec::new()
    }

    pub fn push(&mut self, t: T) {
        self.data[self.size] = MaybeUninit::new(t);
        self.size += 1;
    }

    pub fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter.into_iter() {
            self.push(x);
        }
    }

    pub fn insert(&mut self, pos: usize, t: T) {
        if pos > self.size {
            panic!("oob");
        }

        let mut v = std::mem::replace(&mut self.data[pos], MaybeUninit::new(t));
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

    #[inline(always)]
    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx >= self.size {
            return None;
        }

        Some(unsafe { self.data[idx].assume_init_ref() })
    }

    #[inline(always)]
    pub unsafe fn get_unchecked(&self, idx: usize) -> &T {
        unsafe { self.data[idx].assume_init_ref() }
    }

    pub fn contains(&self, t: &T) -> bool
    where
        T: PartialEq,
    {
        for i in 0..self.size {
            if unsafe { self.get_unchecked(i) } == t {
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
            if unsafe { self.get_unchecked(i) } == t {
                return Ok(i);
            }
        }

        Err(self.size)
    }

    pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a T) -> B,
        B: Ord,
    {
        for i in 0..self.size {
            if f(unsafe { self.get_unchecked(i) }) == *b {
                return Ok(i);
            }
        }
        Err(self.size)
    }

    pub fn as_slice(&self) -> &[T] {
        kani::assume(self.size < VEC_SIZE);
        unsafe { std::mem::transmute(&self.data[..self.size]) }
    }

    pub fn extend_from_slice(&mut self, slice: &[T])
    where
        T: Clone,
    {
        for z in slice {
            self.push(z.clone());
        }
    }

    // dummy retain implementation which does nothing
    pub fn retain<F>(&mut self, mut _f: F)
    where
        F: FnMut(&T) -> bool,
    {
    }

    pub fn to_vec(&self) -> Self
    where
        T: Clone,
    {
        self.clone()
    }

    /// Get `idx`, `kani::assume`ing it is present
    pub fn get_assume(&self, idx: usize) -> &T {
        kani::assert(idx < VEC_SIZE, "idx must be in bounds of vec size");
        let Some(entry) = self.get(idx) else {
            kani::assume(false);
            unreachable!()
        };
        entry
    }
}

impl<T> ops::Deref for Vec<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        kani::assume(self.size < VEC_SIZE);
        unsafe { slice::from_raw_parts(self.data.as_ptr() as *const T, self.size) }
    }
}

impl<T> ops::DerefMut for Vec<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        kani::assume(self.size < VEC_SIZE);
        unsafe { slice::from_raw_parts_mut(self.data.as_mut_ptr() as *mut T, self.size) }
    }
}

impl<'a, T> Iterator for VecIterator<'a, T> {
    type Item = &'a T;

    #[inline(never)]
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        self.idx += 1;

        if idx >= self.vec.size {
            return None;
        }

        kani::assume(idx < VEC_SIZE);

        // SAFETY: We use ManuallyDrop so it's fine to copy this out
        let res = unsafe { self.vec.get_unchecked(idx) };
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

impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = VecIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        VecIntoIterator { vec: self, idx: 0 }
    }
}

impl<T> Iterator for VecIntoIterator<T> {
    type Item = T;

    #[inline(never)]
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        self.idx += 1;

        if idx >= self.vec.size {
            return None;
        }

        kani::assume(idx < VEC_SIZE);

        // SAFETY: We use ManuallyDrop so this is fine
        let res = unsafe { std::ptr::read(self.vec.get_unchecked(idx) as *const T) };
        Some(res)
    }
}

impl<T> FromIterator<T> for Vec<T> {
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

impl<const N: usize, T> TryFrom<Vec<T>> for [T; N] {
    type Error = ();

    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        if vec.len() != N {
            return Err(());
        }
        let vec = ManuallyDrop::new(vec);
        let mut array: [MaybeUninit<T>; N] = [const { MaybeUninit::uninit() }; N];
        let initialized = unsafe {
            // SAFETY: vec.len() == array.len() and don't overlap
            // vec is wrapped in ManuallyDropped and the elements will not be dropped
            array
                .as_mut_ptr()
                .copy_from_nonoverlapping(vec.as_ptr() as *const MaybeUninit<_>, vec.len());
            array.map(|elem| unsafe { elem.assume_init() })
        };
        Ok(initialized)
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
