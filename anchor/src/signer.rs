use crate::prelude::ToAccountInfo;
use otter_solana_program::{account_info::AccountInfo, pubkey::Pubkey, Key};

#[derive(Clone, Debug)]
pub struct Signer<'info> {
    pub info: AccountInfo<'info>,
    pub key: &'info Pubkey,
}

impl<'info> Key for Signer<'info> {
    fn key(&self) -> Pubkey {
        self.info.key()
    }
}

impl<'info> ToAccountInfo<'info> for Signer<'info> {
    fn to_account_info(&self) -> AccountInfo<'info> {
        self.info.clone()
    }
}

#[cfg(any(kani, feature = "kani"))]
impl<'info> kani::Arbitrary for Signer<'info> {
    fn any() -> Self {
        let info = AccountInfo::any();
        let key = info.key;
        Self { info, key }
    }
}
