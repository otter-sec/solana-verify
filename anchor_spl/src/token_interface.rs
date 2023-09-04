use onchor::prelude::*;

use crate::token_2022::TransferChecked;

#[derive(Clone, Debug, Default, PartialEq, AnchorDeserialize, AnchorSerialize)]
#[cfg_attr(any(kani, feature = "kani"), derive(kani::Arbitrary))]
pub struct Mint;

impl AccountDeserialize for Mint {}

impl AccountSerialize for Mint {}

#[derive(Clone, Debug, Default, PartialEq, AnchorDeserialize, AnchorSerialize)]
#[cfg_attr(any(kani, feature = "kani"), derive(kani::Arbitrary))]
pub struct TokenAccount;

impl AccountDeserialize for TokenAccount {}

impl AccountSerialize for TokenAccount {}

#[derive(Clone, Debug, Default, PartialEq, AnchorDeserialize, AnchorSerialize)]
#[cfg_attr(any(kani, feature = "kani"), derive(kani::Arbitrary))]
pub struct TokenInterface;

impl AccountDeserialize for TokenInterface {}

impl AccountSerialize for TokenInterface {}

pub fn transfer_checked<'info>(
    _ctx: CpiContext<'_, '_, '_, 'info, TransferChecked<'info>>,
    _amount: u64,
    _decimals: u8,
) -> Result<()> {
    Ok(())
}
