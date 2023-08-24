use super::pubkey::Pubkey;
use crate::stupid_refcell::{StupidRefMut, StupidRefcell};
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
    pub lamports: StupidRefcell<u64>,
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

    pub fn try_borrow_lamports(&self) -> &u64 {
        self.lamports.borrow()
    }

    pub fn try_borrow_mut_lamports(&mut self) -> StupidRefMut<u64> {
        self.lamports.borrow_mut()
    }

    pub fn realloc(&self, _new_len: usize, _zero_init: bool) -> Result<()> {
        todo!()
    }

    pub fn data_len(&self) -> usize {
        self.data.len()
    }

    pub fn assign(&self, new_owner: &Pubkey) {
        unsafe {
            std::ptr::write_volatile(
                self.owner as *const Pubkey as *mut [u8; 1],
                new_owner.to_bytes(),
            );
        }
    }
}

#[cfg(not(feature = "verify"))]
pub fn next_account_info<'a, 'b, I: Iterator<Item = &'a AccountInfo<'b>>>(
    iter: &mut I,
) -> Result<I::Item> {
    iter.next().ok_or(Error::StdIo)
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

impl<'info> Default for AccountInfo<'info> {
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
