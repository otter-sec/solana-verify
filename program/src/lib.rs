pub mod account_info;
pub mod borsh;
pub mod clock;
pub mod decode_error;
pub mod entrypoint;
pub mod error;
pub mod incinerator;
pub mod instruction;
pub mod native_token;
pub mod program;
pub mod program_error;
pub mod program_memory;
pub mod program_option;
pub mod program_pack;
pub mod pubkey;
pub mod rent;
pub mod stupid_refcell;
pub mod system_instruction;
pub mod system_program;
pub mod sysvar;
pub mod vec {
    pub mod fast;
    pub mod sparse;
}

pub type Result<T> = core::result::Result<T, error::Error>;

pub trait Key {
    fn key(&self) -> pubkey::Pubkey;
}

pub use otter_solana_macro::declare_id;

#[cfg(feature = "verify")]
pub mod verify {
    #[cfg(feature = "kani")]
    pub use kani;

    pub use super::vec;

    pub use super::program_pack::Verify;

    pub use super::vec::fast::Vec;

    pub use otter_solana_macro::{
        account, error_code, invariant, verify, verify_unpackable, Accounts,
    };
}

#[macro_export]
macro_rules! msg {
    ($msg:expr) => {
        // TODO can this be a nop?
        ()
    };
    ($($arg:tt)*) => {
        ()
    };
}

#[macro_export]
macro_rules! vec {
    () => (
        (Vec::new())
    );
    ($elem:expr; $n:expr) => (
        (Vec::from([$elem; $n]))
    );
    ($($x:expr),+ $(,)?) => (
        (Vec::from([$($x),+]))
    );
}

#[macro_export]
macro_rules! entrypoint {
    ($process_instruction:ident) => {
        // nop
    };
}
