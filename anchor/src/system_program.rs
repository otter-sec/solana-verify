use crate::{Id, ToAccountInfos, ToAccountMetas};

use crate::context::CpiContext;
use crate::prelude::Result;
use otter_solana_program::account_info::AccountInfo;
use otter_solana_program::instruction::AccountMeta;
use otter_solana_program::pubkey::Pubkey;

#[derive(Clone, Default)]
#[cfg_attr(any(kani, feature = "kani"), derive(kani::Arbitrary))]
pub struct System;

impl Id for System {
    fn id() -> Pubkey {
        Pubkey::default()
    }
}

#[derive(Debug)]
#[cfg_attr(any(kani, feature = "kani"), derive(kani::Arbitrary))]
pub struct Transfer<'info> {
    pub from: AccountInfo<'info>,
    pub to: AccountInfo<'info>,
}

impl ToAccountMetas for Transfer<'_> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<otter_solana_program::instruction::AccountMeta> {
        let is_signer = is_signer.unwrap_or(self.from.is_signer);
        let from = self.from.to_account_meta(is_signer);
        let to = self.to.to_account_meta(false);
        vec![from, to]
    }
}

impl<'info> ToAccountInfos<'info> for Transfer<'info> {
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        vec![self.from.clone(), self.to.clone()]
    }
}

pub fn transfer<'info>(
    _ctx: CpiContext<'_, '_, '_, 'info, Transfer<'info>>,
    _lamports: u64,
) -> Result<()> {
    Ok(())
}

#[derive(Debug)]
#[cfg_attr(any(kani, feature = "kani"), derive(kani::Arbitrary))]
pub struct CreateAccount<'info> {
    pub from: AccountInfo<'info>,
    pub to: AccountInfo<'info>,
}

impl ToAccountMetas for CreateAccount<'_> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<otter_solana_program::instruction::AccountMeta> {
        let is_signer = is_signer.unwrap_or(self.from.is_signer);
        let from = self.from.to_account_meta(is_signer);
        let to = self.to.to_account_meta(false);
        vec![from, to]
    }
}

impl<'info> ToAccountInfos<'info> for CreateAccount<'info> {
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        vec![self.from.clone(), self.to.clone()]
    }
}

pub fn create_account<'info>(
    _ctx: CpiContext<'_, '_, '_, 'info, CreateAccount<'info>>,
    _lamports: u64,
    _space: u64,
    _program: &Pubkey,
) -> Result<()> {
    Ok(())
}

#[derive(Debug)]
#[cfg_attr(any(kani, feature = "kani"), derive(kani::Arbitrary))]
pub struct Allocate<'info> {
    pub account_to_allocate: AccountInfo<'info>,
}

impl<'info> ToAccountMetas for Allocate<'info> {
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        let is_signer = is_signer.unwrap_or(self.account_to_allocate.is_signer);
        let meta = self.account_to_allocate.to_account_meta(is_signer);
        vec![meta]
    }
}

impl<'info> ToAccountInfos<'info> for Allocate<'info> {
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        vec![self.account_to_allocate.clone()]
    }
}

pub fn allocate<'info>(
    _ctx: CpiContext<'_, '_, '_, 'info, Allocate<'info>>,
    _space: u64,
) -> Result<()> {
    Ok(())
}

#[derive(Debug)]
#[cfg_attr(any(kani, feature = "kani"), derive(kani::Arbitrary))]
pub struct Assign<'info> {
    pub account_to_assign: AccountInfo<'info>,
}

impl<'info> ToAccountMetas for Assign<'info> {
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        let is_signer = is_signer.unwrap_or(self.account_to_assign.is_signer);
        let meta = self.account_to_assign.to_account_meta(is_signer);
        vec![meta]
    }
}

impl<'info> ToAccountInfos<'info> for Assign<'info> {
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        vec![self.account_to_assign.clone()]
    }
}

pub fn assign<'info>(
    _ctx: CpiContext<'_, '_, '_, 'info, Assign<'info>>,
    _program_id: &Pubkey,
) -> Result<()> {
    Ok(())
}
