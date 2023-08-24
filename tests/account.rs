#![cfg(all(feature = "kani", not(feature = "noanchor")))]
use std::fmt::Debug;

use onchor::{
    account::Account, prelude::*, program::Program, signer::Signer, system_program::System,
};

#[account]
#[derive(Clone, Debug)]
#[invariant(self.value == 1)]
pub struct TestAccount {
    pub value: u8,
}

#[derive(Accounts)]
#[instruction(nonce: u8)]
pub struct CreateTransaction<'info> {
    #[account(
        mut,
        seeds = [
            b"test",
        ],
        bump = 1,
        constraint = test_account_mut.account.value == 1 && test_account_init.account.value == 1 && nonce == 1
    )]
    pub test_account_mut: Account<'info, TestAccount>,

    #[account(
        init,
        seeds = [
            b"test",
        ],
        bump,
        space = 8 + mem::size_of::<TestAccount>(),
        payer = creator
    )]
    pub test_account_init: Account<'info, TestAccount>,

    #[account(
        mut,
        seeds = [
            b"test",
        ],
        bump = 1,
        close = creator
    )]
    pub test_account_close: Account<'info, TestAccount>,

    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[test]
fn test_invariants() {
    let mut tx = CreateTransaction::<'_> {
        test_account_mut: Account::<'_, TestAccount> {
            account: TestAccount { value: 1 },
            info: Default::default(),
        },
        test_account_init: Account::<'_, TestAccount> {
            account: TestAccount { value: 1 },
            info: Default::default(),
        },
        test_account_close: Account::<'_, TestAccount> {
            account: TestAccount { value: 1 },
            info: Default::default(),
        },
        creator: Signer::<'_> {
            info: Default::default(),
            key: &Default::default(),
        },
        system_program: Default::default(),
    };
    assert!(tx.__pre_invariants());

    fn initialize(tx: &mut CreateTransaction<'_>) {
        tx.test_account_init.account.value = 1;
    }

    initialize(&mut tx);

    assert!(tx.__post_invariants());
}
