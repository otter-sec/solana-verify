use onchor::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub struct Mint;

impl AccountDeserialize for Mint {}

impl AccountSerialize for Mint {}

#[derive(Clone, Debug, Default, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub struct TokenAccount;

impl AccountDeserialize for TokenAccount {}

impl AccountSerialize for TokenAccount {}

#[derive(Clone, Debug, Default, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub struct TokenInterface;

impl AccountDeserialize for TokenInterface {}

impl AccountSerialize for TokenInterface {}
