use std::marker::PhantomData;

use otter_solana_program::pubkey::Pubkey;
pub use otter_solana_program::Key;
use otter_solana_program::{account_info::AccountInfo, error::Error, Result};

use crate::{Owner, ToAccountInfo, ToAccountMetas, AccountMeta, ToAccountInfos, prelude::FastVec};
use core::cell::{Ref, RefMut, RefCell};

#[derive(Clone)]
pub struct AccountLoader<'info, T: Owner> {
    acc_info: &'info AccountInfo<'info>,
    phantom: PhantomData<&'info T>,
}

impl<'info, T: Owner> AccountLoader<'info, T> {
    pub fn new(acc_info: &'info AccountInfo<'info>) -> Self {
        Self { acc_info, phantom: PhantomData }
    }

     #[inline(never)]
    pub fn try_from(acc_info: &'info AccountInfo<'info>) -> Result<AccountLoader<'info, T>> {
        if acc_info.owner != &T::owner() {
            return Err(Error::Generic);
        }
        Ok(Self::new(acc_info))
    }

    #[inline(never)]
    pub fn try_from_unchecked(acc_info: &'info AccountInfo<'info>) -> AccountLoader<'info, T> {
        Self::new(acc_info)
    }

    #[inline(never)]
    pub fn close(&self, _recipient: AccountInfo<'info>) -> Result<()> {
        // No-op close operation
        Ok(())
    }
}

impl<'info, T: Owner> Key for AccountLoader<'info, T> {
    fn key(&self) -> Pubkey {
        *self.acc_info.key
    }
}

#[cfg(any(kani, feature = "kani"))]
impl<'info, T: Owner> kani::Arbitrary for AccountLoader<'info, T> {
    fn any() -> Self {
        Self {
            acc_info: kani::any(),
            phantom: PhantomData,
        }
    }
}


#[cfg(any(kani, feature = "kani"))]
impl<'info, T: Owner + kani::Arbitrary> AccountLoader<'info, T> {
    #[inline(never)]
    pub fn load(&self) -> Result<Ref<T>> {
        let x = Box::new(RefCell::new(T::any()));
        Ok(Box::leak(x).borrow())
    }

    #[inline(never)]
    pub fn load_mut(&self) -> Result<RefMut<T>> {
        let x = Box::new(RefCell::new(T::any()));
        Ok(Box::leak(x).borrow_mut())
    }

    #[inline(never)]
    pub fn load_init(&self) -> Result<RefMut<T>> {
        let x = Box::new(RefCell::new(T::any()));
        Ok(Box::leak(x).borrow_mut())
    }
}


impl<'info, T: Owner> ToAccountInfo<'info> for AccountLoader<'info, T> {
    fn to_account_info(&self) -> AccountInfo<'info> {
        // Create a new Anchor AccountInfo from your custom AccountInfo's fields
        AccountInfo {
            key: self.acc_info.key,
            is_signer: self.acc_info.is_signer,
            is_writable: self.acc_info.is_writable,
            lamports: self.acc_info.lamports,
            data: self.acc_info.data,
            owner: self.acc_info.owner,
            executable: self.acc_info.executable,
            rent_epoch: self.acc_info.rent_epoch,
        }
    }
}

impl<'info, T: Owner> ToAccountMetas for AccountLoader<'info, T> {
    fn to_account_metas(&self, is_signer: Option<bool>) -> FastVec<AccountMeta> {
        let is_signer = is_signer.unwrap_or(self.acc_info.is_signer);
        let meta = match self.acc_info.is_writable {
            false => AccountMeta::new_readonly(*self.acc_info.key, is_signer),
            true => AccountMeta::new(*self.acc_info.key, is_signer),
        };
        vec![meta].into()
    }
}

impl<'info, T: Owner> ToAccountInfos<'info> for AccountLoader<'info, T> {
    fn to_account_infos(&self) -> FastVec<AccountInfo<'info>> {
        vec![self.acc_info.clone()].into()
    }
}
