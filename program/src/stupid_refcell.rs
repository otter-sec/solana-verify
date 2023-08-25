use std::cell::{BorrowError, BorrowMutError};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct StupidRefCell<T> {
    x: T,
}
impl<T> StupidRefCell<T> {
    pub fn new(x: T) -> Self {
        Self { x }
    }
}

pub struct StupidRefMut<'a, T> {
    x: &'a mut T,
}

#[allow(clippy::should_implement_trait)]
impl<T> StupidRefCell<T>
where
    T: Copy + Clone + std::convert::Into<u64>,
{
    pub fn borrow(&self) -> &T {
        &self.x
    }

    pub fn try_borrow(&self) -> Result<&T, BorrowError> {
        Ok(self.borrow())
    }

    pub fn borrow_mut(&self) -> StupidRefMut<T> {
        StupidRefMut::new(&self.x)
    }

    pub fn try_borrow_mut(&self) -> Result<StupidRefMut<T>, BorrowMutError> {
        Ok(self.borrow_mut())
    }
}

impl<T> Clone for StupidRefCell<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self { x: self.x.clone() }
    }
}

impl<T> Copy for StupidRefCell<T> where T: Copy {}

impl<T> Default for StupidRefCell<T>
where
    T: Default,
{
    fn default() -> Self {
        Self { x: T::default() }
    }
}

#[cfg(any(kani, feature = "kani"))]
impl<T> kani::Arbitrary for StupidRefCell<T>
where
    T: kani::Arbitrary,
{
    fn any() -> Self {
        Self { x: kani::any() }
    }
}

// This is intentionally cursed to accomodate Rc<RefCell<&'a mut u64>> in AccountInfo
#[allow(invalid_reference_casting)]
impl<T> StupidRefMut<'_, T> {
    fn new(x: &T) -> Self {
        Self {
            x: unsafe { &mut *(x as *const T as *mut T) },
        }
    }
}

// This is intentionally cursed to accomodate Rc<RefCell<&'a mut u64>> in AccountInfo
impl<'a, T> Deref for StupidRefMut<'a, T> {
    type Target = &'a mut T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: this is safe as long as no mut borrows have occurred
        &self.x
    }
}

// This is intentionally cursed to accomodate Rc<RefCell<&'a mut u64>> in AccountInfo
impl<'a, T> DerefMut for StupidRefMut<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut &'a mut T {
        // SAFETY: this is safe as long as no concurrent borrows have occurred
        &mut self.x
    }
}
