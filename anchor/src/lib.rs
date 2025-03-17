#![feature(const_trait_impl)]

extern crate core;

pub mod account;
pub mod account_loader;
pub mod context;
pub mod interface;
pub mod program;
pub mod signer;
pub mod system_program;
pub mod sysvar;

use std::{
    collections::{BTreeMap, BTreeSet},
    io::Write,
};

pub use otter_solana_program as solana_program;
use otter_solana_program::{account_info::AccountInfo, instruction::AccountMeta, pubkey::Pubkey};
pub use otter_solana_program::Key;

pub type Result<T> = std::result::Result<T, solana_program::error::Error>;

// Re-export Error from solana_program
pub use solana_program::error::{self, Error};


pub mod accounts {
    pub use crate::account::{self, Account};
    pub use crate::account_loader::{self, AccountLoader};
}

pub trait ZeroCopy : Discriminator {}

pub trait Discriminator {
    const DISCRIMINATOR: [u8; 8];
    fn discriminator() -> [u8; 8] {
        Self::DISCRIMINATOR
    }
}

// pub trait Bump {
//     fn seed(&self) -> u8;
// }
pub use context::{Bumps};
pub use otter_solana_macro::{error_code, declare_id};

pub use ::borsh::{BorshDeserialize as AnchorDeserialize, BorshSerialize as AnchorSerialize};

pub mod __private {
    pub use ::bytemuck;
}

// Roughly following anchor-lang
// see: https://github.com/coral-xyz/anchor/blob/master/lang/src/lib.rs#L235-L266
pub mod prelude {
    pub use std::borrow::{Borrow, BorrowMut};
    pub use std::ops::{Deref, DerefMut};

    pub use ::borsh::{
        self, BorshDeserialize as AnchorDeserialize, BorshSerialize as AnchorSerialize,
    };

    pub use otter_solana_macro::{
        access_control, account, declare_id, error_code, helper_fn, invariant, program, Accounts,
        InitSpace, zero_copy, stub
    };

    pub use crate::account::{self, Account};
    pub use crate::context::{self, Context, CpiContext};
    pub use crate::interface::{Interface, InterfaceAccount};
    pub use crate::program::Program;
    pub use crate::signer::{self, Signer};
    pub use crate::account_loader::{self, AccountLoader};

    pub use super::{
        err, error, require, require_eq, require_neq, require_gt, require_gte, require_keys_eq, require_keys_neq,
        AccountDeserialize, AccountSerialize, Accounts, AccountsClose, AccountsExit, Id, Owner,
        Space, ToAccountInfo, ToAccountInfos, ToAccountMetas
    };
    pub use crate::system_program::{self, System};
    pub use crate::sysvar::Sysvar;

    pub use otter_solana_program as solana_program;
    pub use solana_program::account_info::{next_account_info, AccountInfo};
    pub use solana_program::clock::Clock;
    pub use solana_program::collections::hashmap::HashMap;
    pub use solana_program::error::{Error};
    pub use solana_program::program_error::{ProgramError};
    pub use solana_program::instruction::AccountMeta;
    pub use solana_program::pubkey::Pubkey;
    pub use solana_program::rent::Rent;
    pub use solana_program::string::String;
    pub use solana_program::vec::fast::Vec as FastVec;
    pub use solana_program::Key;
    pub use solana_program::Result;
    pub use solana_program::{entrypoint, msg};

    pub use thiserror;

    #[cfg(any(kani, feature = "kani"))]
    pub use kani::{self, Arbitrary};

    // TODO: maybe fix this?
    // pub use crate::{
    //     solana_program::*,
    //     solana_types::{next_account_info, Result},
    //     vec::fast::Vec,
    // };

    // TODO: fill in more types
}

// TODO make this use ThisError somehow
#[macro_export]
macro_rules! err {
    ($v:expr $(,)?) => {
        Err(anchor_lang::Error::Generic)
    };
}

#[macro_export]
macro_rules! error {
    ($error:expr) => {
        anchor_lang::Error::Generic
    };
}

#[macro_export]
macro_rules! msg {
    ($msg:expr) => {};
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! require {
    ($invariant:expr, $error:tt $(,)?) => {
        if !($invariant) {
            return Err(anchor_lang;:Error::Generic);
        }
    };
    ($invariant:expr, $error:expr $(,)?) => {
        if !($invariant) {
            return Err(anchor_lang::Error::Generic);
        }
    };
    // when no error is provided
    ($invariant:expr $(,)?) => {
        if !($invariant) {
            return Err(anchor_lang::Error::Generic);
        }
    };
}

#[macro_export]
macro_rules! require_eq {
    ($val_1:expr, $val_2:expr, $error:tt $(,)?) => {
        if $val_1 != $val_2 {
            return Err(solana_program::error::Error::Generic);
        }
    };
    ($val_1:expr, $val_2:expr, $error:expr $(,)?) => {
        if $val_1 != $val_2 {
            return Err(solana_program::error::Error::Generic);
        }
    };
    // when no error is provided
    ($val_1:expr, $val_2:expr $(,)?) => {
        if $val_1 != $val_2 {
            return Err(solana_program::error::Error::Generic);
        }
    };
}

#[macro_export]
macro_rules! require_neq {
    ($val_1:expr, $val_2:expr, $error:tt $(,)?) => {
        if $val_1 == $val_2 {
            return Err(solana_program::error::Error::Generic);
        }
    };
    ($val_1:expr, $val_2:expr, $error:expr $(,)?) => {
        if $val_1 == $val_2 {
            return Err(solana_program::error::Error::Generic);
        }
    };
    // when no error is provided
    ($val_1:expr, $val_2:expr $(,)?) => {
        if $val_1 == $val_2 {
            return Err(solana_program::error::Error::Generic);
        }
    };
}

#[macro_export]
macro_rules! require_keys_eq {
    ($key_1:expr, $key_2:expr, $error:tt $(,)?) => {
        if $key_1 != $key_2 {
            return Err(solana_program::error::Error::Generic);
        }
    };
    ($key_1:expr, $key_2:expr, $error:expr $(,)?) => {
        if $key_1 != $key_2 {
            return Err(solana_program::error::Error::Generic);
        }
    };
    // when no error is provided
    ($key_1:expr, $key_2:expr $(,)?) => {
        if $key_1 != $key_2 {
            return Err(solana_program::error::Error::Generic);
        }
    };
}

#[macro_export]
macro_rules! require_keys_neq {
    ($key_1:expr, $key_2:expr, $error:tt $(,)?) => {
        if $key_1 == $key_2 {
            return Err(solana_program::error::Error::Generic);
        }
    };
    ($key_1:expr, $key_2:expr, $error:expr $(,)?) => {
        if $key_1 == $key_2 {
            return Err(solana_program::error::Error::Generic);
        }
    };
    // when no error is provided
    ($key_1:expr, $key_2:expr $(,)?) => {
        if $key_1 == $key_2 {
            return Err(solana_program::error::Error::Generic);
        }
    };
}

#[macro_export]
macro_rules! require_gt {
    // When the error code is provided
    ($value1: expr, $value2: expr, $error_code: expr $(,)?) => {
        if $value1 <= $value2 {
            return Err(solana_program::error::Error::Generic);
        }
    };
    // When the error code is not provided
    ($value1: expr, $value2: expr $(,)?) => {
        if $value1 <= $value2 {
            return Err(solana_program::error::Error::Generic);
        }
    };
}

#[macro_export]
macro_rules! require_gte {
    ($value1: expr, $value2: expr, $error_code: expr $(,)?) => {
        if $value1 < $value2 {
            return Err(solana_program::error::Error::Generic);
        }
    };
    ($value1: expr, $value2: expr $(,)?) => {
        if $value1 < $value2 {
            return Err(solana_program::error::Error::Generic);
        }
    };
}

/// Transformation to an `AccountInfo` struct.
pub trait ToAccountInfo<'info> {
    fn to_account_info(&self) -> AccountInfo<'info>;
}

impl<'info, T> ToAccountInfo<'info> for T
where
    T: AsRef<AccountInfo<'info>>,
{
    fn to_account_info(&self) -> AccountInfo<'info> {
        self.as_ref().clone()
    }
}

pub trait AccountSerialize {
    fn try_serialize<W: Write>(&self, _writer: &mut W) -> Result<()> {
        Ok(())
    }
}

pub trait AccountDeserialize: Sized {
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self> {
        Self::try_deserialize_unchecked(buf)
    }

    /// Deserializes account data without checking the account discriminator.
    /// This should only be used on account initialization, when the bytes of
    /// the account are zeroed.
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self>;
}

pub trait Accounts<'info, B>: ToAccountMetas + ToAccountInfos<'info> + Sized {
    /// Returns the validated accounts struct. What constitutes "valid" is
    /// program dependent. However, users of these types should never have to
    /// worry about account substitution attacks. For example, if a program
    /// expects a `Mint` account from the SPL token program  in a particular
    /// field, then it should be impossible for this method to return `Ok` if
    /// any other account type is given--from the SPL token program or elsewhere.
    ///
    /// `program_id` is the currently executing program. `accounts` is the
    /// set of accounts to construct the type from. For every account used,
    /// the implementation should mutate the slice, consuming the used entry
    /// so that it cannot be used again.
    fn try_accounts(
        program_id: &Pubkey,
        accounts: &mut &'info [AccountInfo<'info>],
        ix_data: &[u8],
        bumps: &mut B,
        reallocs: &mut BTreeSet<Pubkey>,
    ) -> otter_solana_program::Result<Self>;
}

pub trait AccountsClose<'a> {}

pub trait AccountsExit<'info>: ToAccountMetas + ToAccountInfos<'info> {
    /// `program_id` is the currently executing program.
    fn exit(&self, _program_id: &Pubkey) -> otter_solana_program::Result<()> {
        // no-op
        Ok(())
    }
}

pub trait Owner {
    fn owner() -> Pubkey;
}

pub trait Arbitrary {
    fn any() -> Self;
}

pub trait ToAccountInfos<'info> {
    fn to_account_infos(&self) -> solana_program::vec::fast::Vec<AccountInfo<'info>>;
}

pub trait ToAccountMetas {
    fn to_account_metas(&self, is_signer: Option<bool>) -> solana_program::vec::fast::Vec<AccountMeta>;
}

pub trait InstructionData {
    fn data(&self) -> solana_program::vec::sparse::Vec<u8>;
}

pub trait Id {
    fn id() -> Pubkey;
}

pub trait Space {
    const INIT_SPACE: usize;
}

pub mod idl {
    pub const IDL_IX_TAG: u64 = 0x0a69e9a778bcf440;
    pub const IDL_IX_TAG_LE: [u8; 8] = IDL_IX_TAG.to_le_bytes();
}
