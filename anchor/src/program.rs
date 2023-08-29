use std::marker::PhantomData;

use crate::{prelude::Result, ToAccountInfo};
use otter_solana_program::{account_info::AccountInfo, pubkey::Pubkey};

#[derive(Clone)]
pub struct Program<'info, T> {
    info: AccountInfo<'info>,
    _phantom: PhantomData<T>,
}

impl<'info, T> ToAccountInfo<'info> for Program<'info, T> {
    fn to_account_info(&self) -> AccountInfo<'info> {
        self.info.clone()
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

#[cfg(any(kani, feature = "kani"))]
impl<'info, T> kani::Arbitrary for Program<'info, T>
where
    T: kani::Arbitrary,
{
    fn any() -> Self {
        Self {
            _account: kani::any(),
            info: kani::any(),
        }
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
