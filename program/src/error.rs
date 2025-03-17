#[derive(Debug, thiserror::Error)]
pub enum Error {
    // Instructions
    // 100 - 8 byte instruction identifier not provided
    #[error("8 byte instruction identifier not provided")]
    InstructionMissing,
    // 101 - Fallback functions are not supported
    #[error("Fallback functions are not supported")]
    InstructionFallbackNotFound,
    // 102 - The program could not deserialize the given instruction
    #[error("The program could not deserialize the given instruction")]
    InstructionDidNotDeserialize,
    // 103 - The program could not serialize the given instruction
    #[error("The program could not serialize the given instruction")]
    InstructionDidNotSerialize,

    // IDL instructions
    // 1000 - The program was compiled without idl instructions
    #[error("The program was compiled without idl instructions")]
    IdlInstructionStub,
    // 1001 - Invalid program given to the IDL instruction
    #[error("Invalid program given to the IDL instruction")]
    IdlInstructionInvalidProgram,

    // Constraints
    // 2000 - A mut constraint was violated
    #[error("A mut constraint was violated")]
    ConstraintMut,
    // 2001 - A has one constraint was violated
    #[error("A has one constraint was violated")]
    ConstraintHasOne,
    // 2002 - A signer constraint was violated
    #[error("A signer constraint was violated")]
    ConstraintSigner,
    // 2003 - A raw constraint was violated
    #[error("A raw constraint was violated")]
    ConstraintRaw,
    // 2004 - An owner constraint was violated
    #[error("An owner constraint was violated")]
    ConstraintOwner,
    // 2005 - A rent exemption constraint was violated
    #[error("A rent exemption constraint was violated")]
    ConstraintRentExempt,
    // 2006 - A seeds constraint was violated
    #[error("A seeds constraint was violated")]
    ConstraintSeeds,
    // 2007 - An executable constraint was violated
    #[error("An executable constraint was violated")]
    ConstraintExecutable,
    // 2008 - A state constraint was violated
    #[error("A state constraint was violated")]
    ConstraintState,
    // 2009 - An associated constraint was violated
    #[error("An associated constraint was violated")]
    ConstraintAssociated,
    // 2010 - An associated init constraint was violated
    #[error("An associated init constraint was violated")]
    ConstraintAssociatedInit,
    // 2011 - A close constraint was violated
    #[error("A close constraint was violated")]
    ConstraintClose,
    // 2012 - An address constraint was violated
    #[error("An address constraint was violated")]
    ConstraintAddress,
    // 2013 - Expected zero account discriminant
    #[error("Expected zero account discriminant")]
    ConstraintZero,
    // 2014 - A token mint constraint was violated
    #[error("A token mint constraint was violated")]
    ConstraintTokenMint,
    // 2015 - A token owner constraint was violated
    #[error("A token owner constraint was violated")]
    ConstraintTokenOwner,
    // The mint mint is intentional -> a mint authority for the mint.
    //
    // 2016 - A mint mint authority constraint was violated
    #[error("A mint mint authority constraint was violated")]
    ConstraintMintMintAuthority,
    // 2017 - A mint freeze authority constraint was violated
    #[error("A mint freeze authority constraint was violated")]
    ConstraintMintFreezeAuthority,
    // 2018 - A mint decimals constraint was violated
    #[error("A mint decimals constraint was violated")]
    ConstraintMintDecimals,
    // 2019 - A space constraint was violated
    #[error("A space constraint was violated")]
    ConstraintSpace,

    // Require
    // 2500 - A require expression was violated
    #[error("A require expression was violated")]
    RequireViolated,
    // 2501 - A require_eq expression was violated
    #[error("A require_eq expression was violated")]
    RequireEqViolated,
    // 2502 - A require_keys_eq expression was violated
    #[error("A require_keys_eq expression was violated")]
    RequireKeysEqViolated,
    // 2503 - A require_neq expression was violated
    #[error("A require_neq expression was violated")]
    RequireNeqViolated,
    // 2504 - A require_keys_neq expression was violated
    #[error("A require_keys_neq expression was violated")]
    RequireKeysNeqViolated,
    // 2505 - A require_gt expression was violated
    #[error("A require_gt expression was violated")]
    RequireGtViolated,
    // 2506 - A require_gte expression was violated
    #[error("A require_gte expression was violated")]
    RequireGteViolated,

    // Accounts.
    // 3000 - The account discriminator was already set on this account
    #[error("The account discriminator was already set on this account")]
    AccountDiscriminatorAlreadySet,
    // 3001 - No 8 byte discriminator was found on the account
    #[error("No 8 byte discriminator was found on the account")]
    AccountDiscriminatorNotFound,
    // 3002 - 8 byte discriminator did not match what was expected
    #[error("8 byte discriminator did not match what was expected")]
    AccountDiscriminatorMismatch,
    // 3003 - Failed to deserialize the account
    #[error("Failed to deserialize the account")]
    AccountDidNotDeserialize,
    // 3004 - Failed to serialize the account
    #[error("Failed to serialize the account")]
    AccountDidNotSerialize,
    // 3005 - Not enough account keys given to the instruction
    #[error("Not enough account keys given to the instruction")]
    AccountNotEnoughKeys,
    // 3006 - The given account is not mutable
    #[error("The given account is not mutable")]
    AccountNotMutable,
    // 3007 - The given account is owned by a different program than expected
    #[error("The given account is owned by a different program than expected")]
    AccountOwnedByWrongProgram,
    // 3008 - Program ID was not as expected
    #[error("Program ID was not as expected")]
    InvalidProgramId,
    // 3009 - Program account is not executable
    #[error("Program account is not executable")]
    InvalidProgramExecutable,
    // 3010 - The given account did not sign
    #[error("The given account did not sign")]
    AccountNotSigner,
    // 3011 - The given account is not owned by the system program
    #[error("The given account is not owned by the system program")]
    AccountNotSystemOwned,
    // 3012 - The program expected this account to be already initialized
    #[error("The program expected this account to be already initialized")]
    AccountNotInitialized,
    // 3013 - The given account is not a program data account
    #[error("The given account is not a program data account")]
    AccountNotProgramData,
    // 3014 - The given account is not the associated token account
    #[error("The given account is not the associated token account")]
    AccountNotAssociatedTokenAccount,
    // 3015 - The given public key does not match the required sysvar
    #[error("The given public key does not match the required sysvar")]
    AccountSysvarMismatch,
    // 3016 - The account reallocation exceeds the MAX_PERMITTED_DATA_INCREASE limit
    #[error("The account reallocation exceeds the MAX_PERMITTED_DATA_INCREASE limit")]
    AccountReallocExceedsLimit,
    // 3017 - The account was duplicated for more than one reallocation
    #[error("The account was duplicated for more than one reallocation")]
    AccountDuplicateReallocs,

    // State.
    // 4000 - The given state account does not have the correct address
    #[error("The given state account does not have the correct address")]
    StateInvalidAddress,

    // Miscellaneous
    // 4100 - The declared program id does not match actual program id
    #[error("The declared program id does not match the actual program id")]
    DeclaredProgramIdMismatch,

    // Deprecated
    // 5000 - The API being used is deprecated and should no longer be used
    #[error("The API being used is deprecated and should no longer be used")]
    Deprecated,

    #[error(transparent)]
    StdIo(#[from] std::io::Error),

    #[error(transparent)]
    SystemTime(#[from] std::time::SystemTimeError),

    #[error(transparent)]
    Borrow(#[from] std::cell::BorrowError),

    #[error(transparent)]
    BorrowMut(#[from] std::cell::BorrowMutError),

    #[error("generic error")]
    Generic,

    // Custom errors for the program specific errors.
    #[error("An error occurred: {0}")]
    CustomError(String),
    // BoxError is a transparent error that can be used to wrap any error in a Box.
    #[error(transparent)]
    BoxError(#[from] Box<dyn std::error::Error>),
}


#[derive(Debug, thiserror::Error)]
pub enum ProgramError {
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


pub enum ErrorCode {
    // Instructions
    /// 100 - 8 byte instruction identifier not provided
    InstructionMissing = 100,
    /// 101 - Fallback functions are not supported
    InstructionFallbackNotFound,
    /// 102 - The program could not deserialize the given instruction
    InstructionDidNotDeserialize,
    /// 103 - The program could not serialize the given instruction
    InstructionDidNotSerialize,

    // IDL instructions
    /// 1000 - The program was compiled without idl instructions
    IdlInstructionStub = 1000,
    /// 1001 - Invalid program given to the IDL instruction
    IdlInstructionInvalidProgram,
    /// 1002 - IDL Account must be empty in order to resize
    IdlAccountNotEmpty,

    // Event instructions
    /// 1500 - The program was compiled without `event-cpi` feature
    EventInstructionStub = 1500,

    // Constraints
    /// 2000 - A mut constraint was violated
    ConstraintMut = 2000,
    /// 2001 - A has one constraint was violated
    ConstraintHasOne,
    /// 2002 - A signer constraint was violated
    ConstraintSigner,
    /// 2003 - A raw constraint was violated
    ConstraintRaw,
    /// 2004 - An owner constraint was violated
    ConstraintOwner,
    /// 2005 - A rent exemption constraint was violated
    ConstraintRentExempt,
    /// 2006 - A seeds constraint was violated
    ConstraintSeeds,
    /// 2007 - An executable constraint was violated
    ConstraintExecutable,
    /// 2008 - Deprecated Error, feel free to replace with something else
    ConstraintState,
    /// 2009 - An associated constraint was violated
    ConstraintAssociated,
    /// 2010 - An associated init constraint was violated
    ConstraintAssociatedInit,
    /// 2011 - A close constraint was violated
    ConstraintClose,
    /// 2012 - An address constraint was violated
    ConstraintAddress,
    /// 2013 - Expected zero account discriminant
    ConstraintZero,
    /// 2014 - A token mint constraint was violated
    ConstraintTokenMint,
    /// 2015 - A token owner constraint was violated
    ConstraintTokenOwner,
    /// The mint mint is intentional -> a mint authority for the mint.
    ///
    /// 2016 - A mint mint authority constraint was violated
    ConstraintMintMintAuthority,
    /// 2017 - A mint freeze authority constraint was violated
    ConstraintMintFreezeAuthority,
    /// 2018 - A mint decimals constraint was violated
    ConstraintMintDecimals,
    /// 2019 - A space constraint was violated
    ConstraintSpace,
    /// 2020 - A required account for the constraint is None
    ConstraintAccountIsNone,
    /// The token token is intentional -> a token program for the token account.
    ///
    /// 2021 - A token account token program constraint was violated
    ConstraintTokenTokenProgram,
    /// 2022 - A mint token program constraint was violated
    ConstraintMintTokenProgram,
    /// 2023 - A mint token program constraint was violated
    ConstraintAssociatedTokenTokenProgram,

    // Require
    /// 2500 - A require expression was violated
    RequireViolated = 2500,
    /// 2501 - A require_eq expression was violated
    RequireEqViolated,
    /// 2502 - A require_keys_eq expression was violated
    RequireKeysEqViolated,
    /// 2503 - A require_neq expression was violated
    RequireNeqViolated,
    /// 2504 - A require_keys_neq expression was violated
    RequireKeysNeqViolated,
    /// 2505 - A require_gt expression was violated
    RequireGtViolated,
    /// 2506 - A require_gte expression was violated
    RequireGteViolated,

    // Accounts.
    /// 3000 - The account discriminator was already set on this account
    AccountDiscriminatorAlreadySet = 3000,
    /// 3001 - No 8 byte discriminator was found on the account
    AccountDiscriminatorNotFound,
    /// 3002 - 8 byte discriminator did not match what was expected
    AccountDiscriminatorMismatch,
    /// 3003 - Failed to deserialize the account
    AccountDidNotDeserialize,
    /// 3004 - Failed to serialize the account
    AccountDidNotSerialize,
    /// 3005 - Not enough account keys given to the instruction
    AccountNotEnoughKeys,
    /// 3006 - The given account is not mutable
    AccountNotMutable,
    /// 3007 - The given account is owned by a different program than expected
    AccountOwnedByWrongProgram,
    /// 3008 - Program ID was not as expected
    InvalidProgramId,
    /// 3009 - Program account is not executable
    InvalidProgramExecutable,
    /// 3010 - The given account did not sign
    AccountNotSigner,
    /// 3011 - The given account is not owned by the system program
    AccountNotSystemOwned,
    /// 3012 - The program expected this account to be already initialized
    AccountNotInitialized,
    /// 3013 - The given account is not a program data account
    AccountNotProgramData,
    /// 3014 - The given account is not the associated token account
    AccountNotAssociatedTokenAccount,
    /// 3015 - The given public key does not match the required sysvar
    AccountSysvarMismatch,
    /// 3016 - The account reallocation exceeds the MAX_PERMITTED_DATA_INCREASE limit
    AccountReallocExceedsLimit,
    /// 3017 - The account was duplicated for more than one reallocation
    AccountDuplicateReallocs,

    // Miscellaneous
    /// 4100 - The declared program id does not match actual program id
    DeclaredProgramIdMismatch = 4100,
    /// 4101 - You cannot/should not initialize the payer account as a program account
    TryingToInitPayerAsProgramAccount = 4101,

    // Deprecated
    /// 5000 - The API being used is deprecated and should no longer be used
    Deprecated = 5000,
}

impl Error {
    pub fn with_account_name(self) -> Self { self }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Extract variant name from the type info
        let type_str = std::any::type_name_of_val(self);
        let variant = type_str.split("::").last().unwrap_or("Unknown");
        
        // Write both the message and the variant name
        write!(f, "Error occured: {}", variant)
    }
}


impl From<ErrorCode> for Error {
    fn from(error_code: ErrorCode) -> Self {
        crate::error::Error::CustomError(error_code.to_string())
    }
}

impl From<ProgramError> for Error {
    fn from(value: ProgramError) -> Self {
        crate::error::Error::CustomError(value.to_string())
    }
}
