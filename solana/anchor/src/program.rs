use crate::ToAccountInfo;
use otter_solana_program::account_info::AccountInfo;

pub struct Program<'info, T> {
    _account: T,
    info: AccountInfo<'info>,
}

impl<'info, T> ToAccountInfo<'info> for Program<'info, T> {
    fn to_account_info(&self) -> AccountInfo<'info> {
        self.info.clone()
    }
}

#[cfg(any(kani, feature = "kani"))]
impl<'info, T> kani::Arbitrary for Program<'info, T>
where
    T: kani::Arbitrary,
{
    fn any() -> Self {
        Self {
            _account: kani::any(),
            info: kani::any(),
        }
    }
}

impl<'info, T> Default for Program<'info, T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            _account: Default::default(),
            info: Default::default(),
        }
    }
}
