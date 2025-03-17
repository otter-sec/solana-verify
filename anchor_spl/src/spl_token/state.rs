use onchor::prelude::*;
use onchor::solana_program::program_option::COption;
use crate::spl_token::instruction::{MAX_SIGNERS, MIN_SIGNERS};
use num_enum::TryFromPrimitive;


/// Account data.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, kani::Arbitrary, AnchorSerialize, AnchorDeserialize)]
pub struct Account {
    /// The mint associated with this account
    pub mint: Pubkey,
    /// The owner of this account.
    pub owner: Pubkey,
    /// The amount of tokens this account holds.
    pub amount: u64,
    /// If `delegate` is `Some` then `delegated_amount` represents
    /// the amount authorized by the delegate
    pub delegate: COption<Pubkey>,
    /// The account's state
    pub state: AccountState,
    /// If is_native.is_some, this is a native token, and the value logs the
    /// rent-exempt reserve. An Account is required to be rent-exempt, so
    /// the value is used by the Processor to ensure that wrapped SOL
    /// accounts do not drop below this threshold.
    pub is_native: COption<u64>,
    /// The amount delegated
    pub delegated_amount: u64,
    /// Optional authority to close the account.
    pub close_authority: COption<Pubkey>,
}


impl Account {
    pub const LEN: usize = 165;
}

///
/// Account state.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, TryFromPrimitive, kani::Arbitrary, AnchorSerialize, AnchorDeserialize)]
pub enum AccountState {
    /// Account is not yet initialized
    #[default]
    Uninitialized,
    /// Account is initialized; the account owner and/or delegate may perform
    /// permitted operations on this account
    Initialized,
    /// Account has been frozen by the mint freeze authority. Neither the
    /// account owner nor the delegate are able to perform operations on
    /// this account.
    Frozen,
}

/// Mint data.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, kani::Arbitrary, AnchorSerialize, AnchorDeserialize)]
pub struct Mint {
    /// Optional authority used to mint new tokens. The mint authority may only
    /// be provided during mint creation. If no mint authority is present
    /// then the mint has a fixed supply and no further tokens may be
    /// minted.
    pub mint_authority: COption<Pubkey>,
    /// Total supply of tokens.
    pub supply: u64,
    /// Number of base 10 digits to the right of the decimal place.
    pub decimals: u8,
    /// Is `true` if this structure has been initialized
    pub is_initialized: bool,
    /// Optional authority to freeze token accounts.
    pub freeze_authority: COption<Pubkey>,
}

impl Mint {
    pub const LEN: usize = 82;
}

/// Multisignature data.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub struct Multisig {
    /// Number of signers required
    pub m: u8,
    /// Number of valid signers
    pub n: u8,
    /// Is `true` if this structure has been initialized
    pub is_initialized: bool,
    /// Signer public keys
    pub signers: [Pubkey; MAX_SIGNERS],
}

impl Multisig {
    pub const LEN: usize = 355;
}
