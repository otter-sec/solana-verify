use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use otter_solana_program::{
    account_info::AccountInfo, instruction::AccountMeta, pubkey::Pubkey, Key, Result,
};

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

impl<'info, T> ToAccountInfo<'info> for Interface<'info, T> {
    fn to_account_info(&self) -> AccountInfo<'info> {
        self.0.to_account_info()
    }
}

#[derive(Clone)]
#[cfg_attr(any(kani, feature = "kani"), derive(kani::Arbitrary))]
pub struct InterfaceAccount<'info, T: AccountSerialize + AccountDeserialize + Clone> {
    pub account: Account<'info, T>,
    pub owner: Pubkey,
}

impl<'info, T: AccountSerialize + AccountDeserialize + Clone> ToAccountInfo<'info>
    for InterfaceAccount<'info, T>
{
    fn to_account_info(&self) -> AccountInfo<'info> {
        self.account.to_account_info()
    }
}

impl<'info, T: AccountSerialize + AccountDeserialize + Clone + fmt::Debug> fmt::Debug
    for InterfaceAccount<'info, T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.account.fmt(f)
    }
}

impl<'a, T: AccountSerialize + AccountDeserialize + Clone> InterfaceAccount<'a, T> {
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

impl<'info, T: AccountSerialize + AccountDeserialize + Clone> ToAccountMetas
    for InterfaceAccount<'info, T>
{
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        self.account.to_account_metas(is_signer)
    }
}

impl<'info, T: AccountSerialize + AccountDeserialize + Clone> ToAccountInfos<'info>
    for InterfaceAccount<'info, T>
{
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        self.account.to_account_infos()
    }
}

impl<'info, T: AccountSerialize + AccountDeserialize + Clone> AsRef<T>
    for InterfaceAccount<'info, T>
{
    fn as_ref(&self) -> &T {
        self.account.as_ref()
    }
}

impl<'a, T: AccountSerialize + AccountDeserialize + Clone> Deref for InterfaceAccount<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.account.deref()
    }
}

impl<'a, T: AccountSerialize + AccountDeserialize + Clone> DerefMut for InterfaceAccount<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.account.deref_mut()
    }
}

impl<'info, T: AccountSerialize + AccountDeserialize + Clone> Key for InterfaceAccount<'info, T> {
    fn key(&self) -> Pubkey {
        self.account.key()
    }
}
