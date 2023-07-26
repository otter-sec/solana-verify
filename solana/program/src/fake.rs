use std::{
    cell::RefCell,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::vec::fast::Vec;

pub struct FakeRef<'a, T> {
    _t: RefCell<T>,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> FakeRef<'a, T> {
    pub fn new(t: T) -> Self {
        Self {
            _t: RefCell::new(t),
            _phantom: PhantomData {},
        }
    }
}

impl<'a, T> Deref for FakeRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self._t.as_ptr().as_ref().unwrap() }
    }
}

pub struct MutFakeRef<'a, T> {
    _t: RefCell<T>,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> MutFakeRef<'a, T> {
    pub fn new(t: T) -> Self {
        Self {
            _t: RefCell::new(t),
            _phantom: PhantomData {},
        }
    }
}

impl<'a, T> Deref for MutFakeRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self._t.as_ptr().as_ref().unwrap() }
    }
}

impl<'a, T> DerefMut for MutFakeRef<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self._t.as_ptr().as_mut().unwrap() }
    }
}

pub struct FakeArrRef<'a, T> {
    _t: RefCell<Vec<T>>,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> FakeArrRef<'a, T> {
    pub fn new(t: Vec<T>) -> Self {
        Self {
            _t: RefCell::new(t),
            _phantom: PhantomData {},
        }
    }
}

impl<'a, T> Deref for FakeArrRef<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { self._t.as_ptr().as_ref().unwrap() }
    }
}
