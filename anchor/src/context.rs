use std::marker::PhantomData;

use otter_solana_program::{account_info::AccountInfo, pubkey::Pubkey, vec::fast::Vec};

use crate::{ToAccountInfos, ToAccountMetas};

// Lightweight "btreemap" for bumps
#[derive(Clone, Debug)]
pub struct FakeBumps;

impl FakeBumps {
    pub fn get(&self, _: &str) -> Option<&u8> {
        Some(&0)
    }
}

// Represents a type that can be contained in Account<T>
pub trait AccountType {}

#[derive(Debug)]
pub struct ConcreteContext<'a, 'b, 'c, 'info, T> {
    pub program_id: Pubkey,
    pub accounts: T,
    pub remaining_accounts: Vec<AccountInfo<'info>>,

    _a: PhantomData<&'a u8>,
    _b: PhantomData<&'b u8>,
    _c: PhantomData<&'c u8>,
}

impl<'info, T> ConcreteContext<'_, '_, '_, 'info, T> {
    pub fn to_ctx<'a>(&'a self) -> Context<'a, 'a, 'a, 'info, T> {
        Context {
            program_id: &self.program_id,
            accounts: unsafe { (&self.accounts as *const T as *mut T).as_mut().unwrap() },
            remaining_accounts: &self.remaining_accounts,
            bumps: FakeBumps {},
        }
    }
}

impl<'a, T> From<&'a ConcreteContext<'a, 'a, 'a, 'a, T>> for Context<'a, 'a, 'a, 'a, T> {
    fn from(value: &'a ConcreteContext<'a, 'a, 'a, 'a, T>) -> Self {
        value.to_ctx()
    }
}

#[cfg(any(kani, feature = "kani"))]
impl<'a, 'b, 'c, 'info, T> kani::Arbitrary for ConcreteContext<'a, 'b, 'c, 'info, T>
where
    T: kani::Arbitrary,
{
    fn any() -> Self {
        Self {
            program_id: kani::any(),
            accounts: kani::any(),
            remaining_accounts: Vec::from([]),

            _a: PhantomData {},
            _b: PhantomData {},
            _c: PhantomData {},
        }
    }
}

#[derive(Debug)]
pub struct Context<'a, 'b, 'c, 'info, T> {
    pub program_id: &'a Pubkey,
    pub accounts: &'b mut T,
    pub remaining_accounts: &'c [AccountInfo<'info>],
    pub bumps: FakeBumps,
}

impl<'a, 'b, 'c, 'info, T> Context<'a, 'b, 'c, 'info, T> {
    pub fn new(
        program_id: &'a Pubkey,
        accounts: &'b mut T,
        remaining_accounts: &'c [AccountInfo<'info>],
        bumps: FakeBumps,
    ) -> Self {
        Self {
            program_id,
            accounts,
            remaining_accounts,
            bumps,
        }
    }
}

#[cfg(any(kani, feature = "kani"))]
impl<'a, T> kani::Arbitrary for Context<'a, 'a, 'a, 'a, T>
where
    T: kani::Arbitrary,
{
    fn any() -> Context<'a, 'a, 'a, 'a, T> {
        let concrete: ConcreteContext<'a, 'a, 'a, 'a, T> = kani::any();
        let concrete: &'a ConcreteContext<'a, 'a, 'a, 'a, T> = Box::leak(Box::new(concrete));
        concrete.into()
    }
}

#[derive(Debug)]
pub struct CpiContext<'a, 'b, 'c, 'info, T>
where
    T: ToAccountMetas + ToAccountInfos<'info>,
{
    pub accounts: T,
    pub remaining_accounts: Vec<AccountInfo<'info>>,
    pub program: AccountInfo<'info>,
    pub signer_seeds: &'a [&'b [&'c [u8]]],
}

impl<'a, 'b, 'c, 'info, T> CpiContext<'a, 'b, 'c, 'info, T>
where
    T: ToAccountMetas + ToAccountInfos<'info>,
{
    pub fn new(program: AccountInfo<'info>, accounts: T) -> Self {
        Self {
            accounts,
            program,
            remaining_accounts: Vec::new(),
            signer_seeds: &[],
        }
    }

    #[must_use]
    pub fn new_with_signer(
        program: AccountInfo<'info>,
        accounts: T,
        signer_seeds: &'a [&'b [&'c [u8]]],
    ) -> Self {
        Self {
            accounts,
            program,
            signer_seeds,
            remaining_accounts: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_signer(mut self, signer_seeds: &'a [&'b [&'c [u8]]]) -> Self {
        self.signer_seeds = signer_seeds;
        self
    }

    #[must_use]
    pub fn with_remaining_accounts(mut self, ra: Vec<AccountInfo<'info>>) -> Self {
        self.remaining_accounts = ra;
        self
    }
}

#[cfg(any(kani, feature = "kani"))]
impl<'a, 'b, 'c, 'info, T> kani::Arbitrary for CpiContext<'a, 'b, 'c, 'info, T>
where
    T: kani::Arbitrary + ToAccountMetas + ToAccountInfos<'info>,
{
    fn any() -> Self {
        Self {
            program: kani::any(),
            accounts: kani::any(),
            remaining_accounts: Vec::from([]),
            signer_seeds: &[],
        }
    }
}
