use onchor::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub struct Mint;

impl AccountDeserialize for Mint {}

impl AccountSerialize for Mint {}
