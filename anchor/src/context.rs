use std::marker::PhantomData;

use otter_solana_program::{account_info::AccountInfo, pubkey::Pubkey, vec::fast::Vec};

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

// TODO(hgarrereyn): this is currently really expensive

// pub fn kani_new_array() -> &'static mut [u8] {
//     unsafe {
//         let length: usize = kani::any();
//         kani::assume(length <= SIZE);
//         for i in 0..SIZE {
//             DATA[i] = kani::any();
//         }
//         &mut DATA[0 .. length]
//     }
// }

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
