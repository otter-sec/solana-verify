#![feature(const_trait_impl)]

extern crate core;

pub mod account;
pub mod context;
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

// Roughly following anchor-lang
// see: https://github.com/coral-xyz/anchor/blob/master/lang/src/lib.rs#L235-L266
pub mod prelude {
    pub use std::borrow::{Borrow, BorrowMut};
    pub use std::ops::{Deref, DerefMut};

    pub use ::borsh::{
        self, BorshDeserialize as AnchorDeserialize, BorshSerialize as AnchorSerialize,
    };

    pub use otter_solana_macro::{
        account, declare_id, error_code, invariant, program, Accounts, Arbitrary,
    };

    pub use crate::account::{self, Account};
    pub use crate::context::{self, Context};
    pub use crate::program::Program;
    pub use crate::signer::{self, Signer};

    pub use super::{
        err, AccountDeserialize, AccountSerialize, Accounts, AccountsClose, AccountsExit, Id,
        Owner, ToAccountInfo, ToAccountInfos, ToAccountMetas,
    };
    pub use crate::system_program::System;
    pub use crate::sysvar::Sysvar;

    pub use otter_solana_program as solana_program;
    pub use solana_program::account_info::{next_account_info, AccountInfo};
    pub use solana_program::error::{self, Error, ErrorCode};
    pub use solana_program::instruction::AccountMeta;
    pub use solana_program::pubkey::Pubkey;
    pub use solana_program::rent::Rent;
    pub use solana_program::vec::fast::Vec;
    pub use solana_program::Key;
    pub use solana_program::Result;
    pub use solana_program::{entrypoint, msg};

    pub use thiserror;

    #[cfg(any(kani, feature = "kani"))]
    pub use kani;

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
    ($v:expr) => {
        Err(Error {})
    };
}

// #[macro_export]
// macro_rules! msg {
//     ($msg:expr) => {
//         $crate::log::sol_log($msg)
//     };
//     ($($arg:tt)*) => ($crate::log::sol_log(&format!($($arg)*)));
// }

/// Transformation to an `AccountInfo` struct.
pub trait ToAccountInfo<'info> {
    fn to_account_info(&self) -> AccountInfo<'info>;
}

// impl<'info, T> ToAccountInfo<'info> for T
// where
//     T: AsRef<AccountInfo<'info>>,
// {
//     fn to_account_info(&self) -> AccountInfo<'info> {
//         self.as_ref().clone()
//     }
// }

pub trait AccountSerialize: crate::prelude::AnchorSerialize {
    /// Serializes the account data into `writer`.
    fn try_serialize<W: Write>(&self, writer: &mut W) -> otter_solana_program::Result<()> {
        self.serialize(writer).map_err(|_| crate::prelude::Error)
    }
}

pub trait AccountDeserialize: crate::prelude::AnchorDeserialize {
    /// Deserializes previously initialized account data. Should fail for all
    /// uninitialized accounts, where the bytes are zeroed. Implementations
    /// should be unique to a particular account type so that one can never
    /// successfully deserialize the data of one account type into another.
    /// For example, if the SPL token program were to implement this trait,
    /// it should be impossible to deserialize a `Mint` account into a token
    /// `Account`.
    fn try_deserialize(buf: &mut &[u8]) -> otter_solana_program::Result<Self> {
        Self::try_deserialize_unchecked(buf)
    }

    /// Deserializes account data without checking the account discriminator.
    /// This should only be used on account initialization, when the bytes of
    /// the account are zeroed.
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> otter_solana_program::Result<Self> {
        Self::deserialize(buf).map_err(|_| crate::prelude::Error)
    }
}

pub trait Accounts<'info>: ToAccountMetas + ToAccountInfos<'info> + Sized {
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
        accounts: &mut &[AccountInfo<'info>],
        ix_data: &[u8],
        bumps: &mut BTreeMap<String, u8>,
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

pub trait ToAccountInfos<'info> {
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>>;
}

pub trait ToAccountMetas {
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta>;
}

pub trait Id {
    fn id() -> Pubkey;
}
