use std::{marker::PhantomData, ops::Deref};

use crate::{prelude::Result, ToAccountInfo};
use otter_solana_program::{account_info::AccountInfo, pubkey::Pubkey};

#[derive(Clone)]
#[cfg_attr(any(kani, feature = "kani"), derive(kani::Arbitrary))]
pub struct Program<'info, T> {
    info: AccountInfo<'info>,
    _phantom: PhantomData<T>,
}

impl<'info, T> ToAccountInfo<'info> for Program<'info, T> {
    fn to_account_info(&self) -> AccountInfo<'info> {
        self.info.clone()
    }
}

impl<'info, T> Deref for Program<'info, T> {
    type Target = AccountInfo<'info>;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl<'info, T> AsRef<AccountInfo<'info>> for Program<'info, T> {
    fn as_ref(&self) -> &AccountInfo<'info> {
        self.deref()
    }
}

impl<'a, T> Program<'a, T> {
    pub fn new(info: AccountInfo<'a>) -> Self {
        Self {
            info,
            _phantom: PhantomData,
        }
    }

    pub fn programdata_address(&self) -> Result<Option<Pubkey>> {
        Ok(Some(*self.info.key))
    }
}

impl<'info, T> Default for Program<'info, T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            info: Default::default(),
            _phantom: PhantomData,
        }
    }
}
