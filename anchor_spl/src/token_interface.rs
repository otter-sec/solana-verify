use onchor::prelude::*;
use onchor as anchor_lang;

pub use crate::spl_token_2022;
pub use crate::token_2022::{TransferChecked, transfer, transfer_checked, Transfer};

#[derive(Debug, Default, PartialEq)]
#[repr(transparent)]
#[account]
pub struct Mint(spl_token_2022::state::Mint);

impl Deref for Mint {
    type Target = spl_token_2022::state::Mint;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default, PartialEq)]
#[repr(transparent)]
#[account]
#[invariant()]
pub struct TokenAccount(spl_token_2022::state::Account);

impl Deref for TokenAccount {
    type Target = spl_token_2022::state::Account;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug, Default, PartialEq, AnchorDeserialize, AnchorSerialize)]
#[cfg_attr(any(kani, feature = "kani"), derive(kani::Arbitrary))]
pub struct TokenInterface;


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
