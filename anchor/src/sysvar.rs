use crate::ToAccountInfo;
use otter_solana_program::{account_info::AccountInfo, rent::Rent};
use crate::prelude::{FastVec, ToAccountMetas, ToAccountInfos, AccountMeta};

pub struct Sysvar<'info, T> {
    info: AccountInfo<'info>,
    account: T,
}

impl<'info, T: Clone> Clone for Sysvar<'info, T> {
    fn clone(&self) -> Self {
        Self {
            info: self.info.clone(),
            account: self.account.clone()
        }
    }
}

impl<'info, T> ToAccountInfo<'info> for Sysvar<'info, T> {
    fn to_account_info(&self) -> AccountInfo<'info> {
        self.info
    }
}

impl<'info> Sysvar<'info, Rent> {
    pub fn minimum_balance(&self, data_len: usize) -> u64 {
        self.account.minimum_balance(data_len)
    }
}

#[cfg(any(kani, feature = "kani"))]
impl<'info, T: kani::Arbitrary> kani::Arbitrary for Sysvar<'info, T> {
    fn any() -> Self {
        Self {
            info: kani::any(),
            account: kani::any(),
        }
    }
}

impl<'info, T> ToAccountMetas for Sysvar<'info, T> {
    fn to_account_metas(&self, _is_signer: Option<bool>) -> FastVec<AccountMeta> {
        vec![AccountMeta::new_readonly(*self.info.key, false)].into()
    }
}

impl<'info, T> ToAccountInfos<'info> for Sysvar<'info, T> {
    fn to_account_infos(&self) -> FastVec<AccountInfo<'info>> {
        vec![self.info.clone()].into()
    }
}
