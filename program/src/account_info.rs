use std::cell::{BorrowError, BorrowMutError};

use super::pubkey::Pubkey;
use crate::instruction::AccountMeta;
use crate::stupid_refcell::{StupidRefCell, StupidRefMut};
use crate::{pubkey::KEYS, vec::sparse::Vec, Key, Result};

#[cfg(not(feature = "verify"))]
use crate::error::Error;

#[cfg(any(kani, feature = "kani"))]
use crate::pubkey::kani_new_pubkey;

#[derive(Clone, Debug)]
pub struct AccountInfo<'a> {
    pub key: &'a Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
    pub lamports: StupidRefCell<u64>,
    pub data: Vec<u8>,
    pub owner: &'a Pubkey,
    pub executable: bool,
    pub rent_epoch: bool, //Epoch,
}

impl<'a> AccountInfo<'a> {
    pub fn try_borrow_data(&self) -> Result<&Vec<u8>> {
        Ok(&self.data)
    }

    pub fn try_borrow_mut_data(&self) -> Result<&Vec<u8>> {
        self.try_borrow_data()
    }

    pub fn lamports(&self) -> u64 {
        *self.lamports.borrow()
    }

    pub fn try_borrow_lamports(&self) -> std::result::Result<u64, BorrowError> {
        self.lamports.try_borrow().copied()
    }

    pub fn try_borrow_mut_lamports(
        &mut self,
    ) -> std::result::Result<StupidRefMut<u64>, BorrowMutError> {
        self.lamports.try_borrow_mut()
    }

    pub fn realloc(&self, _new_len: usize, _zero_init: bool) -> Result<()> {
        Ok(())
    }

    pub fn data_len(&self) -> usize {
        self.data.len()
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
        }
    }
}
