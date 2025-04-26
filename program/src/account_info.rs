use std::cell::{BorrowError, BorrowMutError};

use super::pubkey::Pubkey;
use crate::instruction::AccountMeta;
use crate::stupid_refcell::{StupidRef, StupidRefCell, StupidRefMut};
pub use crate::Key;
use crate::{pubkey::KEYS, vec::sparse::Vec, Result};

#[cfg(not(feature = "verify"))]
use crate::error::Error;

#[cfg(any(kani, feature = "kani"))]
use crate::pubkey::kani_new_pubkey;

use core::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use std::any::{Any, TypeId};

use dyn_clone::DynClone;

#[derive(Clone, Copy, Debug)]
pub struct AccountInfo<'a> {
    pub key: &'a Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
    pub lamports: StupidRefCell<u64>,
    pub data: Vec<u8>,
    pub owner: &'a Pubkey,
    pub executable: bool,
    pub rent_epoch: bool, //Epoch,

    // extra fields for verification
    pub deserialized: *mut Option<RefCell<*mut dyn shared::Invariant<AccountInfo<'static>>>>,
    pub disallow_mut: bool
}

impl AccountInfo<'static> {
    pub fn check_invariant(&self) -> Option<()> {
        let slf: &'static AccountInfo<'static> = unsafe { std::mem::transmute(self) };
        slf.get_dyn()?.check_invariant();
        Some(())
    }

    pub fn check_transition_invariant(&self, old: &Self, account_infos: &[Self]) -> Option<()> {
        let a = self.get_dyn()?;
        if let Some(old) = old.get_dyn() {
            kani::assert(
                a.type_id() == old.type_id(),
                "old and new are different types",
            );
        };
        a.check_transition_invariant(old, account_infos);
        Some(())
    }

    pub fn get_dyn(&self) -> Option<&'static dyn shared::Invariant<AccountInfo<'static>>> {
        let deserialized = unsafe { &*self.deserialized }.as_ref()?;
        let t = unsafe { &**deserialized.try_borrow().ok()? };
        Some(t as &dyn shared::Invariant<_>)
    }

    pub fn as_invariant(&self) -> &dyn shared::Invariant<AccountInfo<'static>> {
        let obj = self.get_dyn();
        kani::assert(obj.is_some(), "`deserialized` not initialized in `as_invariant`");
        obj.unwrap()
    }
}

impl<'a> AccountInfo<'a> {
    pub fn as_account<
        T: kani::Arbitrary + Clone + shared::Invariant<AccountInfo<'static>> + 'static,
    >(
        &self,
    ) -> Ref<T> {
        self.assert_init_as::<T>();

        let r = unsafe { &*self.deserialized }.as_ref().unwrap();
        let t = unsafe { &**r.borrow() };

        Ref::map(r.borrow(), |x| {
            unsafe { &**x }.as_any().downcast_ref::<T>().unwrap()
        })
    }

    pub fn as_account_mut<
        T: kani::Arbitrary + Clone + shared::Invariant<AccountInfo<'static>> + 'static,
    >(
        &self,
    ) -> RefMut<T> {
        kani::assert(!self.disallow_mut, "raw AccountInfo data borrowed mutably");
        self.assert_init_as::<T>();

        let r = unsafe { &*self.deserialized }.as_ref().unwrap();
        let t = unsafe { &**r.borrow() };

        RefMut::map(r.borrow_mut(), |x| {
            unsafe { &mut **x }
                .as_any_mut()
                .downcast_mut::<T>()
                .unwrap()
        })
    }

    pub fn as_account_ref<
        T: kani::Arbitrary + Clone + shared::Invariant<AccountInfo<'static>> + 'static,
    >(
        &self,
    ) -> &T {
        self.assert_init_as::<T>();

        let r = unsafe { &*self.deserialized }.as_ref().unwrap();
        let t = unsafe { &**r.borrow() };

        t.as_any().downcast_ref::<T>().unwrap()
    }

    pub fn maybe_as_account<
        T: kani::Arbitrary + Clone + shared::Invariant<AccountInfo<'static>> + 'static,
    >(
        &self,
    ) -> Option<&T> {
        // kani::assert(unsafe { &mut *self.deserialized }.is_some(), "maybe_as_account has no data :(");
        // kani::assert(unsafe { &**((&mut *self.deserialized).as_ref().unwrap().borrow()) }.as_any().downcast_ref::<T>().is_some(), "maybe_as_account wrong type :/");
        unsafe { &mut *self.deserialized }
            .as_ref()
            .map(|x| {
                let t = unsafe { &**x.borrow() };
                t.as_any().downcast_ref::<T>()
            })
            .flatten()
    }

    pub fn as_account_ref_mut<
        T: kani::Arbitrary + Clone + shared::Invariant<AccountInfo<'static>> + 'static,
    >(
        &self,
    ) -> &mut T {
        kani::assert(!self.disallow_mut, "raw AccountInfo data borrowed mutably");
        self.assert_init_as::<T>();

        let r = unsafe { &*self.deserialized }.as_ref().unwrap();
        let mut t = unsafe { &mut **r.borrow() };

        t.as_any_mut().downcast_mut::<T>().unwrap()
    }

    pub fn init_as<
        T: Sized + kani::Arbitrary + Clone + shared::Invariant<AccountInfo<'static>> + 'static,
    >(
        &self,
    ) {
        let mut d = unsafe { &mut *self.deserialized };
        assert!(d.is_none());
        let t: T = kani::any();
        *d = Some(RefCell::new(Box::leak(Box::new(t)) as *mut _));
    }

    pub fn init_with<
        T: Sized + kani::Arbitrary + Clone + shared::Invariant<AccountInfo<'static>> + 'static,
    >(
        &self,
        value: T,
    ) {
        let mut d = unsafe { &mut *self.deserialized };
        assert!(d.is_none());
        *d = Some(RefCell::new(Box::leak(Box::new(value)) as *mut _));
    }

    pub fn assert_init_as<
        T: Sized + kani::Arbitrary + Clone + shared::Invariant<AccountInfo<'static>> + 'static,
    >(
        &self,
    ) {
        let r = unsafe { &*self.deserialized }.as_ref();
        kani::assert(r.is_some(), "All AccountInfo must be initialized with a type using #[assume_types()]");

        let t = unsafe { &**r.unwrap().borrow() };
        kani::assert(t.as_any().is::<T>(), "AccountInfo was deserialized as a different type than it was initialized!");
    }

    pub fn is_initialized(&self) -> bool {
        unsafe {
            (*self.deserialized).is_some()
        }
    }
}

impl<'a> AccountInfo<'a> {
    pub fn signer_key(&self) -> Option<&Pubkey> {
        if self.is_signer {
            Some(self.key)
        } else {
            None
        }
    }

    pub fn clone_data(&mut self) {
        if self.disallow_mut { return; }

        let mut d = unsafe { &mut *self.deserialized };

        let inner = d.as_ref().map(|x| {
            let ptr = *x.borrow();
            let cloned_t: &mut dyn shared::Invariant<AccountInfo<'static>> =
                Box::leak(dyn_clone::clone_box(unsafe { &*ptr }));
            RefCell::new(cloned_t as *mut _)
        });
        self.deserialized = Box::leak(Box::new(inner)) as *mut _
    }

    pub fn unsigned_key(&self) -> &Pubkey {
        self.key
    }

    pub fn lamports(&self) -> u64 {
        *self.lamports.borrow()
    }

    pub fn try_borrow_lamports(&self) -> std::result::Result<StupidRef<u64>, BorrowError> {
        self.lamports.try_borrow()
    }

    pub fn data_len(&self) -> usize {
        self.data.borrow().len()
    }

    pub fn try_data_len(&self) -> std::result::Result<usize, BorrowError> {
        Ok(self.data.borrow().len())
    }

    pub fn data_is_empty(&self) -> bool {
        self.data.borrow().is_empty()
    }

    pub fn try_data_is_empty(&self) -> std::result::Result<bool, BorrowError> {
        Ok(self.data.borrow().is_empty())
    }

    pub fn try_borrow_data(&self) -> Result<&[u8]> {
        Ok(&self.data)
    }

    pub fn try_borrow_mut_data(&self) -> Result<&[u8]> {
        self.try_borrow_data()
    }

    pub fn try_borrow_mut_lamports(
        &mut self,
    ) -> std::result::Result<StupidRefMut<u64>, BorrowMutError> {
        self.lamports.try_borrow_mut()
    }

    pub fn realloc(&self, _new_len: usize, _zero_init: bool) -> Result<()> {
        Ok(())
    }

    pub fn to_account_meta(&self, is_signer: bool) -> AccountMeta {
        if self.is_writable {
            AccountMeta::new(*self.key, is_signer)
        } else {
            AccountMeta::new_readonly(*self.key, is_signer)
        }
    }

    #[allow(invalid_reference_casting)]
    pub fn assign(&self, new_owner: &Pubkey) {
        unsafe {
            std::ptr::write_volatile(
                self.owner as *const Pubkey as *mut [u8; 1],
                new_owner.to_bytes(),
            );
        }
    }
}

impl<'a> AsRef<AccountInfo<'a>> for AccountInfo<'a> {
    fn as_ref(&self) -> &AccountInfo<'a> {
        self
    }
}

#[cfg(not(feature = "verify"))]
pub fn next_account_info<'a, 'b, I: Iterator<Item = &'a AccountInfo<'b>>>(
    iter: &mut I,
) -> Result<I::Item> {
    iter.next().ok_or(Error::InstructionMissing)
}

#[cfg(feature = "verify")]
use crate::program_error::ProgramError;

#[cfg(feature = "verify")]
pub fn next_account_info<'a, 'b, I: Iterator<Item = &'a AccountInfo<'b>>>(
    iter: &mut I,
) -> core::result::Result<I::Item, ProgramError> {
    iter.next().ok_or(ProgramError::NotEnoughAccountKeys)
}

impl<'a> Key for AccountInfo<'a> {
    fn key(&self) -> Pubkey {
        *self.key
    }
}

#[cfg(any(kani, feature = "kani"))]
impl<'info> kani::Arbitrary for AccountInfo<'info> {
    fn any() -> Self {
        Self {
            key: kani_new_pubkey(),
            is_signer: kani::any(),
            is_writable: kani::any(),
            lamports: kani::any(),
            data: kani::any(),
            owner: kani_new_pubkey(),
            executable: kani::any(),
            rent_epoch: kani::any(),
            deserialized: Box::leak(Box::new(None)) as *mut _,
            disallow_mut: false
        }
    }
}

impl Default for AccountInfo<'_> {
    fn default() -> Self {
        Self {
            key: unsafe { KEYS.get(0).unwrap() },
            is_signer: bool::default(),
            is_writable: bool::default(),
            lamports: Default::default(),
            data: Vec::<u8>::default(),
            owner: unsafe { KEYS.get(0).unwrap() },
            executable: bool::default(),
            rent_epoch: bool::default(),
            deserialized: Box::leak(Box::new(None)) as *mut _,
            disallow_mut: false
        }
    }
}

