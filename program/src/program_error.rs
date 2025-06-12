use super::decode_error::DecodeError;
use num_traits::FromPrimitive;

// pub type ProgramError = Error;

/// Reasons the program may fail
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ProgramError {
    /// Allows on-chain programs to implement program-specific error types and see them returned
    /// by the Solana runtime. A program-specific error may be any type that is represented as
    /// or serialized to a u32 integer.
    #[error("Custom program error: {0:#x}")]
    Custom(u32),
    #[error("The arguments provided to a program instruction were invalid")]
    InvalidArgument,
    #[error("An instruction's data contents was invalid")]
    InvalidInstructionData,
    #[error("An account's data contents was invalid")]
    InvalidAccountData,
    #[error("An account's data was too small")]
    AccountDataTooSmall,
    #[error("An account's balance was too small to complete the instruction")]
    InsufficientFunds,
    #[error("The account did not have the expected program id")]
    IncorrectProgramId,
    #[error("A signature was required but not found")]
    MissingRequiredSignature,
    #[error("An initialize instruction was sent to an account that has already been initialized")]
    AccountAlreadyInitialized,
    #[error("An attempt to operate on an account that hasn't been initialized")]
    UninitializedAccount,
    #[error("The instruction expected additional account keys")]
    NotEnoughAccountKeys,
    #[error("Failed to borrow a reference to account data, already borrowed")]
    AccountBorrowFailed,
    #[error("Length of the seed is too long for address generation")]
    MaxSeedLengthExceeded,
    #[error("Provided seeds do not result in a valid address")]
    InvalidSeeds,
    #[error("IO Error: {0}")]
    BorshIoError(String),
    #[error("An account does not have enough lamports to be rent-exempt")]
    AccountNotRentExempt,
    #[error("Unsupported sysvar")]
    UnsupportedSysvar,
    #[error("Provided owner is not allowed")]
    IllegalOwner,
    #[error("Accounts data allocations exceeded the maximum allowed per transaction")]
    MaxAccountsDataAllocationsExceeded,
    #[error("Account data reallocation was invalid")]
    InvalidRealloc,
    #[error("Instruction trace length exceeded the maximum allowed per transaction")]
    MaxInstructionTraceLengthExceeded,
    #[error("Builtin programs must consume compute units")]
    BuiltinProgramsMustConsumeComputeUnits,
    #[error("Invalid account owner")]
    InvalidAccountOwner,
    #[error("Program arithmetic overflowed")]
    ArithmeticOverflow,
}

const BUILTIN_BIT_SHIFT: usize = 32;
macro_rules! to_builtin {
    ($error:expr) => {
        ($error as u64) << BUILTIN_BIT_SHIFT
    };
}

pub const CUSTOM_ZERO: u64 = to_builtin!(1);
pub const INVALID_ARGUMENT: u64 = to_builtin!(2);
pub const INVALID_INSTRUCTION_DATA: u64 = to_builtin!(3);
pub const INVALID_ACCOUNT_DATA: u64 = to_builtin!(4);
pub const ACCOUNT_DATA_TOO_SMALL: u64 = to_builtin!(5);
pub const INSUFFICIENT_FUNDS: u64 = to_builtin!(6);
pub const INCORRECT_PROGRAM_ID: u64 = to_builtin!(7);
pub const MISSING_REQUIRED_SIGNATURES: u64 = to_builtin!(8);
pub const ACCOUNT_ALREADY_INITIALIZED: u64 = to_builtin!(9);
pub const UNINITIALIZED_ACCOUNT: u64 = to_builtin!(10);
pub const NOT_ENOUGH_ACCOUNT_KEYS: u64 = to_builtin!(11);
pub const ACCOUNT_BORROW_FAILED: u64 = to_builtin!(12);
pub const MAX_SEED_LENGTH_EXCEEDED: u64 = to_builtin!(13);
pub const INVALID_SEEDS: u64 = to_builtin!(14);
pub const BORSH_IO_ERROR: u64 = to_builtin!(15);
pub const ACCOUNT_NOT_RENT_EXEMPT: u64 = to_builtin!(16);
pub const UNSUPPORTED_SYSVAR: u64 = to_builtin!(17);
pub const ILLEGAL_OWNER: u64 = to_builtin!(18);
pub const MAX_ACCOUNTS_DATA_ALLOCATIONS_EXCEEDED: u64 = to_builtin!(19);
pub const INVALID_ACCOUNT_DATA_REALLOC: u64 = to_builtin!(20);
pub const MAX_INSTRUCTION_TRACE_LENGTH_EXCEEDED: u64 = to_builtin!(21);
pub const BUILTIN_PROGRAMS_MUST_CONSUME_COMPUTE_UNITS: u64 = to_builtin!(22);

pub trait PrintProgramError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive;
}

impl PrintProgramError for ProgramError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        // nop
    }
}

#[cfg(not(kani))]
impl From<ProgramError> for crate::error::Error {
    fn from(value: ProgramError) -> Self {
        crate::error::Error::CustomError(value.to_string())
    }
}

#[cfg(kani)]
impl From<ProgramError> for crate::error::Error {
    fn from(value: ProgramError) -> Self {
        crate::error::Error::Generic
    }
}
