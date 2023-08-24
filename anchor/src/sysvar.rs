use crate::ToAccountInfo;
use otter_solana_program::{account_info::AccountInfo, rent::Rent};

pub struct Sysvar<'info, T> {
    info: AccountInfo<'info>,
    account: T,
}

impl<'info, T> ToAccountInfo<'info> for Sysvar<'info, T> {
    fn to_account_info(&self) -> AccountInfo<'info> {
        self.info.clone()
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
