use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use otter_solana_program::{
    account_info::AccountInfo, instruction::AccountMeta, pubkey::Pubkey, vec::fast::Vec, Result
};
pub use otter_solana_program::Key;

use crate::{
    prelude::{Account, Program},
    AccountDeserialize, AccountSerialize, ToAccountInfo, ToAccountInfos, ToAccountMetas,
};

#[derive(Clone)]
#[cfg_attr(any(kani, feature = "kani"), derive(kani::Arbitrary))]
pub struct Interface<'info, T>(Program<'info, T>);

impl<'a, T> Interface<'a, T> {
    pub fn new(info: AccountInfo<'a>) -> Self {
        Self(Program::new(info))
    }

    pub fn programdata_address(&self) -> Result<Option<Pubkey>> {
        self.0.programdata_address()
    }
}

impl<'info, T> Deref for Interface<'info, T> {
    type Target = AccountInfo<'info>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'info, T> AsRef<AccountInfo<'info>> for Interface<'info, T> {
    fn as_ref(&self) -> &AccountInfo<'info> {
        &self.0
    }
}

#[derive(Clone)]
#[cfg_attr(any(kani, feature = "kani"), derive(kani::Arbitrary))]
pub struct InterfaceAccount<'info, T: AccountSerialize + AccountDeserialize + Clone + kani::Arbitrary + 'static> {
    pub account: Account<'info, T>,
    pub owner: Pubkey,
}

impl<'info, T: AccountSerialize + AccountDeserialize + Clone + kani::Arbitrary + 'static> ToAccountInfo<'info>
    for InterfaceAccount<'info, T>
{
    fn to_account_info(&self) -> AccountInfo<'info> {
        self.account.to_account_info()
    }
}

impl<'info, T: AccountSerialize + AccountDeserialize + Clone + kani::Arbitrary + fmt::Debug> fmt::Debug
    for InterfaceAccount<'info, T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.account.fmt(f)
    }
}

impl<'a, T: AccountSerialize + AccountDeserialize + shared::Invariant<AccountInfo<'static>> + Clone + kani::Arbitrary + 'static> InterfaceAccount<'a, T> {
    pub fn new(info: AccountInfo<'a>, account: T) -> Self {
        let owner = *info.owner;
        Self {
            account: Account::new(info, account),
            owner,
        }
    }

    pub fn reload(&mut self) -> Result<()> {
        self.account.reload()
    }

    pub fn into_inner(self) -> T {
        self.account.into_inner()
    }

    pub fn set_inner(&mut self, inner: T) {
        self.account.set_inner(inner);
    }
}

impl<'info, T: AccountSerialize + AccountDeserialize + Clone + kani::Arbitrary + shared::Invariant<AccountInfo<'static>> + 'static> ToAccountMetas
    for InterfaceAccount<'info, T>
{
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        self.account.to_account_metas(is_signer)
    }
}

impl<'info, T: AccountSerialize + AccountDeserialize + Clone + kani::Arbitrary + shared::Invariant<AccountInfo<'static>> + 'static> ToAccountInfos<'info>
    for InterfaceAccount<'info, T>
{
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        self.account.to_account_infos()
    }
}

impl<'info, T: AccountSerialize + AccountDeserialize + Clone + kani::Arbitrary + shared::Invariant<AccountInfo<'static>> + 'static> AsRef<T>
    for InterfaceAccount<'info, T>
{
    fn as_ref(&self) -> &T {
        self.account.as_ref()
    }
}

impl<'a, T: AccountSerialize + AccountDeserialize + Clone + kani::Arbitrary + shared::Invariant<AccountInfo<'static>> + 'static> Deref for InterfaceAccount<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.account.deref()
    }
}

impl<'a, T: AccountSerialize + AccountDeserialize + Clone + kani::Arbitrary + shared::Invariant<AccountInfo<'static>> + 'static> DerefMut for InterfaceAccount<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.account.deref_mut()
    }
}

impl<'info, T: AccountSerialize + AccountDeserialize + Clone + kani::Arbitrary + shared::Invariant<AccountInfo<'static>> + 'static> Key for InterfaceAccount<'info, T> {
    fn key(&self) -> Pubkey {
        self.account.key()
    }
}
