use std::any::Any;
use std::ops::{Deref, DerefMut};

use crate::{prelude::AnchorDeserialize, ToAccountInfos, ToAccountMetas, AccountSerialize, AccountDeserialize};
use crate::{Owner, ToAccountInfo};
use otter_solana_program::{
    account_info::AccountInfo, error::Error, instruction::AccountMeta, pubkey::Pubkey, Key, Result, vec::fast::Vec
};
use shared::Invariant;

use std::marker::PhantomData;
use std::cell::{Ref, RefMut, RefCell};

#[derive(Debug)]
#[cfg_attr(any(kani, feature = "kani"), derive(kani::Arbitrary))]
pub struct Account<'info, T> {
    // pub account: T, NOTE: This is now kept in `info`
    pub info: AccountInfo<'info>,
    pub phantom: PhantomData<T>,
}

impl<'info, T> Clone for Account<'info, T> {
    fn clone(&self) -> Self {
        let mut info = self.info.clone();
        info.clone_data();

        Self {
            info,
            phantom: PhantomData
        }
    }
}

impl<'a, T: kani::Arbitrary + Clone + shared::Invariant<AccountInfo<'static>> + 'static> Account<'a, T> {
    pub fn new(info: AccountInfo<'a>, account: T) -> Account<'a, T> {
        *info.as_account_mut::<T>() = account;
        Self { info, phantom: PhantomData }
    }

    pub fn reload(&mut self) -> Result<()> {
        unimplemented!();
        Ok(())
    }

    pub fn set_inner(&mut self, inner: T) {
        *self.info.as_account_mut::<T>() = inner;
    }

    pub fn close(self, _info: AccountInfo<'_>) -> Result<()> {
        Ok(())
    }

    pub fn account_for_verification(&self) -> Ref<T> {
        self.info.as_account::<T>()
    }
}

impl<T: Invariant<AccountInfo<'static>> + kani::Arbitrary + Clone + 'static> Invariant<AccountInfo<'static>> for Account<'static, T> {
    fn as_any(&self) -> &dyn Any {
        self as _
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as _
    }

    fn check_invariant(&self) {
        self.info.as_account::<T>().check_invariant()
    }

    fn check_transition_invariant(&self, other: &dyn Invariant<AccountInfo<'static>>, remaining: &[AccountInfo<'static>]) {
        self.info.as_account::<T>().check_transition_invariant(other, remaining)
    }
}

impl <'a, T: Clone + kani::Arbitrary + shared::Invariant<AccountInfo<'static>> + Clone + 'static> Account<'a, T> {
    pub fn into_inner(self) -> T {
        self.info.as_account::<T>().clone()
    }
}

impl<'a, T: AnchorDeserialize + Owner + kani::Arbitrary + shared::Invariant<AccountInfo<'static>> + Clone + 'static> Account<'a, T> {
    #[inline(never)]
    pub fn try_from(info: &AccountInfo<'a>) -> Result<Account<'a, T>> {
        if
        /* info.owner == &system_program::ID && */
        info.lamports() == 0 {
            // return Err(ErrorCode::AccountNotInitialized.into());
            return Err(Error::AccountDidNotDeserialize);
        }
        if info.owner != &T::owner() {
            return Err(Error::AccountDidNotDeserialize);
        }
        let mut data: &[u8] = info.try_borrow_data()?;
        Ok(Account::new(
            *info,
            T::deserialize(&mut data).map_err(|_| Error::AccountDidNotDeserialize)?,
        ))
    }

    #[inline(never)]
    pub fn try_from_unchecked(info: &AccountInfo<'a>) -> Account<'a, T> {
        Self::try_from(info).unwrap()
    }
}

impl<'info, T> ToAccountMetas for Account<'info, T> {
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        let is_signer = is_signer.unwrap_or(self.info.is_signer);
        let meta = match self.info.is_writable {
            false => AccountMeta::new_readonly(*self.info.key, is_signer),
            true => AccountMeta::new(*self.info.key, is_signer),
        };
        vec![meta].into()
    }
}

impl<'info, T: kani::Arbitrary + Clone + 'static> ToAccountInfos<'info> for Account<'info, T> {
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        vec![self.info].into()
    }
}

impl<'a, T: kani::Arbitrary + Clone + shared::Invariant<AccountInfo<'static>> + 'static> Deref for Account<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.info.as_account_ref::<T>()
    }
}

impl<'a, T: kani::Arbitrary + Clone + shared::Invariant<AccountInfo<'static>> + 'static> AsRef<T> for Account<'a, T> {
    fn as_ref(&self) -> &T {
        self.info.as_account_ref::<T>()
    }
}

impl<'a, T: kani::Arbitrary + Clone + Invariant<AccountInfo<'static>> + 'static> DerefMut for Account<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        #[cfg(feature = "anchor-debug")]
        if !self.info.is_writable {
            solana_program::msg!("The given Account is not mutable");
            panic!();
        }
        self.info.as_account_ref_mut::<T>()
    }
}

impl<'info, T> Key for Account<'info, T> {
    fn key(&self) -> Pubkey {
        *self.info.key
    }
}

impl<'info, T> TryFrom<&AccountInfo<'info>> for Account<'info, T>
where
    T: AnchorDeserialize + Owner,
{
    type Error = Error;

    fn try_from(info: &AccountInfo<'info>) -> Result<Self> {
        Self::try_from(info)
    }
}

impl ToAccountMetas for AccountInfo<'_> {
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        let is_signer = is_signer.unwrap_or(self.is_signer);
        let meta = match self.is_writable {
            false => AccountMeta::new_readonly(*self.key, is_signer),
            true => AccountMeta::new(*self.key, is_signer),
        };
        vec![meta].into()
    }
}

impl<'info> ToAccountInfos<'info> for AccountInfo<'info> {
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        vec![self.clone()].into()
    }
}

impl<'info, T: ToAccountInfos<'info>> ToAccountInfos<'info> for Option<T> {
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        self.as_ref()
            .map_or_else(Vec::new, |account| account.to_account_infos())
    }
}

impl<'info, T: AccountSerialize + AccountDeserialize + Clone + 'static> AsRef<AccountInfo<'info>>
    for Account<'info, T>
{
    fn as_ref(&self) -> &AccountInfo<'info> {
        &self.info
    }
}
