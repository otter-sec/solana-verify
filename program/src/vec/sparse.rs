use std::{
    marker::PhantomData,
    ops::{self, Index},
};

use crate::Result;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Vec<T> {
    pub size: usize,
    pub t: PhantomData<T>,
}

impl<T: Default> Default for Vec<T> {
    fn default() -> Self {
        Self {
            size: 0,
            t: PhantomData,
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

    pub fn new_with_size(size: usize) -> Vec<T> {
        Self {
            size,
            t: PhantomData,
        }
    }

    pub fn push(&mut self, _: T) {
        panic!("not allowed");
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn contains(&self, _: &T) -> bool
    where
        T: PartialEq,
    {
        panic!("not allowed");
    }

    pub fn sort(&mut self) {
        // nothing
    }

    pub fn dedup(&mut self) {
        // todo
    }

    pub fn remove(&mut self, _: usize)
    where
        T: Copy,
    {
        panic!("not allowed");
    }

    pub fn binary_search(&self, _: &T) -> Result<usize>
    where
        T: PartialEq,
    {
        panic!("not allowed");
    }

    pub fn as_slice(&self) -> SparseSlice<T> {
        SparseSlice::<T> {
            len: self.size,
            t: Default::default(),
        }
    }
    // We do not implement Borrow<[T]>
    // The point is that you will break the type checker if you try to use
    // a sparse::Vec's data outside of specific functions we have created
    // (e.g. our pack/unpack implementation)
    pub fn borrow(&self) -> SparseSlice<T> {
        self.as_slice()
    }

    // HACK: Put it in a box since normally AccountInfo's data is
    // a Rc<RefCell<&mut [u8]>>
    //
    // SAFETY: no writes will actually occur (we have no data!) so this being non-mut is fine
    pub fn borrow_mut(&self) -> Box<SparseSlice<T>> {
        Box::new(self.as_slice())
    }
}

impl<T> ops::Deref for Vec<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        panic!("not allowed");
    }
}

impl<T> ops::DerefMut for Vec<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        panic!("not allowed");
    }
}

impl<T: Default> FromIterator<T> for Vec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_: I) -> Self {
        panic!("not allowed");
    }
}

impl<T: Default, const N: usize> From<[T; N]> for Vec<T> {
    fn from(_: [T; N]) -> Vec<T> {
        panic!("not allowed");
    }
}

#[cfg(any(kani, feature = "kani"))]
impl<T: kani::Arbitrary + Default> kani::Arbitrary for Vec<T> {
    fn any() -> Self {
        Self {
            size: kani::any(),
            t: PhantomData,
        }
    }
}

// SparseSlice represents a slice of this "sparse" vec.
// This is special for a number of reasons:
//
// - The sparse vector implementation is used on types
// that have a Vec but whose contents we do not want to
// reason about. AccountInfo's data is one such example,
// since we will be reasoning at pack/unpack time instead.
//
// - Thus, the SparseSlice actually has no data.
// - The type itself, (due to its current usage for account
// data) can now only be passed to specific functions, and
// this is checked by the type checker! This serves as a cheap
// way to ensure that the account data is only used for packing
// and unpacking.

pub struct SparseSlice<T> {
    pub len: usize,
    pub t: PhantomData<T>,
}

impl<T> SparseSlice<T> {
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn len(&self) -> usize {
        self.len
    }
}

impl Index<usize> for SparseSlice<usize> {
    type Output = usize;

    fn index(&self, _: usize) -> &usize {
        &0usize
    }
}

impl Index<usize> for Box<SparseSlice<usize>> {
    type Output = usize;

    fn index(&self, _: usize) -> &usize {
        &0usize
    }
}
