use crate::prelude::{AnchorDeserialize, AnchorSerialize};

use crate::{AccountDeserialize, AccountSerialize, Id};

use otter_solana_program::pubkey::Pubkey;

#[cfg(any(kani, feature = "kani"))]
use otter_solana_macro::Arbitrary;

#[derive(Clone, Default, AnchorSerialize, AnchorDeserialize)]
#[cfg_attr(any(kani, feature = "kani"), derive(Arbitrary))]
pub struct System;

impl Id for System {
    fn id() -> Pubkey {
        Pubkey::default()
    }
}

impl AccountSerialize for System {}
impl AccountDeserialize for System {}
