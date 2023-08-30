use crate::Id;

use otter_solana_program::account_info::AccountInfo;
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
