use onchor::prelude::{AccountMeta, Vec, account, invariant};
use onchor::solana_program::account_info::AccountInfo;

use onchor::solana_program::pubkey::Pubkey;
pub use onchor::system_program::Transfer;
use onchor::context::CpiContext;
use onchor::{solana_program, Result, ToAccountInfos, ToAccountMetas, AccountDeserialize, AccountSerialize, AnchorSerialize, AnchorDeserialize};

use crate::token_2022::TransferChecked;

use {kani, kani::Arbitrary};
use onchor as anchor_lang;

solana_program::declare_id!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");


pub fn transfer<'info>(
    _ctx: CpiContext<'_, '_, '_, 'info, Transfer<'info>>,
    _amount: u64,
) -> Result<()> {
    Ok(())
}

pub fn transfer_checked<'info>(
    _ctx: CpiContext<'_, '_, '_, 'info, TransferChecked<'info>>,
    _amount: u64,
    _decimals: u8,
) -> Result<()> {
    Ok(())
}


#[cfg_attr(any(kani, feature = "kani"), derive(kani::Arbitrary))]
pub struct Burn<'info> {
    pub mint: AccountInfo<'info>,
    pub from: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
}


impl ToAccountMetas for Burn<'_> {
    fn to_account_metas(&self, _: Option<bool>) -> Vec<AccountMeta> {
        vec![self.mint.to_account_meta(false), self.from.to_account_meta(false), self.authority.to_account_meta(false)].into()
    }
}

impl<'info> ToAccountInfos<'info> for Burn<'info> {
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        vec![self.mint, self.from, self.authority].into()
    }
}

pub fn burn<'info>(_: CpiContext<'_, '_, '_, 'info, Burn<'info>>, _: u64) -> Result<()> {
    Ok(())
}

#[derive(Debug)]
#[cfg_attr(any(kani, feature = "kani"), derive(kani::Arbitrary))]
pub struct MintTo<'info> {
    pub mint: AccountInfo<'info>,
    pub to: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
}

impl ToAccountMetas for MintTo<'_> {
    fn to_account_metas(&self, _: Option<bool>) -> Vec<AccountMeta> {
        vec![self.mint.to_account_meta(false), self.to.to_account_meta(false), self.authority.to_account_meta(false)].into()
    }
}

impl<'info> ToAccountInfos<'info> for MintTo<'info> {
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        vec![self.mint, self.to, self.authority].into()
    }
}

pub fn mint_to<'info>(_: CpiContext<'_, '_, '_, 'info, MintTo<'info>>, _: u64) -> Result<()> {
    Ok(())
}

#[derive(Clone)]
#[cfg_attr(any(kani, feature = "kani"), derive(kani::Arbitrary))]
pub struct Token;

#[derive(Clone, Debug, Default, PartialEq)]
#[account]
#[invariant()]
pub struct Mint;

#[derive(Clone, Debug, Default, PartialEq)]
#[account]
#[invariant()]
pub struct TokenAccount;


pub mod accessor {
    use super::*;

    // u8 = 1 byte starts from 3rd byte of data
    pub fn amount(account: &AccountInfo) -> Result<u64> {
        let bytes = account.try_borrow_data()?;
        let mut amount_bytes = [0u8; 8];
        amount_bytes.copy_from_slice(&bytes[2..10]);
        Ok(u64::from_le_bytes(amount_bytes))
    }

    // 1st byte of data is the mint key
    pub fn mint(account: &AccountInfo) -> Result<Pubkey> {
        let bytes = account.try_borrow_data()?;
        Ok(Pubkey::new_from_array([bytes[0]]))
    }

    // 2nd byte of data is the authority key
    pub fn authority(account: &AccountInfo) -> Result<Pubkey> {
        let bytes = account.try_borrow_data()?;
        Ok(Pubkey::new_from_array([bytes[1]]))
    }
}
