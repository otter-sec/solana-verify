use num_enum::{TryFromPrimitive, IntoPrimitive, FromPrimitive};
use crate::spl_token_2022::state::*;
use crate::spl_token::state::Multisig;
use onchor::prelude::{ProgramError, Pubkey, FastVec};
use onchor::solana_program::vec::sparse::SparseSlice;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum AccountType {
    Uninitialized = 0,
    Mint = 1,
    Account = 2,
}

/// Extensions that can be applied to mints or accounts.  Mint extensions must
/// only be applied to mint accounts, and account extensions must only be
/// applied to token holding accounts.
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, TryFromPrimitive, IntoPrimitive, kani::Arbitrary, Default)]
pub enum ExtensionType {
    /// Used as padding if the account size would otherwise be 355, same as a
    /// multisig
    #[default]
    Uninitialized,
    /// Includes transfer fee rate info and accompanying authorities to withdraw
    /// and set the fee
    TransferFeeConfig,
    /// Includes withheld transfer fees
    TransferFeeAmount,
    /// Includes an optional mint close authority
    MintCloseAuthority,
    /// Auditor configuration for confidential transfers
    ConfidentialTransferMint,
    /// State for confidential transfers
    ConfidentialTransferAccount,
    /// Specifies the default Account::state for new Accounts
    DefaultAccountState,
    /// Indicates that the Account owner authority cannot be changed
    ImmutableOwner,
    /// Require inbound transfers to have memo
    MemoTransfer,
    /// Indicates that the tokens from this mint can't be transferred
    NonTransferable,
    /// Tokens accrue interest over time,
    InterestBearingConfig,
    /// Locks privileged token operations from happening via CPI
    CpiGuard,
    /// Includes an optional permanent delegate
    PermanentDelegate,
    /// Indicates that the tokens in this account belong to a non-transferable
    /// mint
    NonTransferableAccount,
    /// Mint requires a CPI to a program implementing the "transfer hook"
    /// interface
    TransferHook,
    /// Indicates that the tokens in this account belong to a mint with a
    /// transfer hook
    TransferHookAccount,
    /// Includes encrypted withheld fees and the encryption public that they are
    /// encrypted under
    ConfidentialTransferFeeConfig,
    /// Includes confidential withheld transfer fees
    ConfidentialTransferFeeAmount,
    /// Mint contains a pointer to another account (or the same account) that
    /// holds metadata
    MetadataPointer,
    /// Mint contains token-metadata
    TokenMetadata,
    /// Mint contains a pointer to another account (or the same account) that
    /// holds group configurations
    GroupPointer,
    /// Mint contains token group configurations
    TokenGroup,
    /// Mint contains a pointer to another account (or the same account) that
    /// holds group member configurations
    GroupMemberPointer,
    /// Mint contains token group member configurations
    TokenGroupMember,
    /// Mint allowing the minting and burning of confidential tokens
    ConfidentialMintBurn,
    /// Tokens whose UI amount is scaled by a given amount
    ScaledUiAmount,
    /// Tokens where minting / burning / transferring can be paused
    Pausable,
    /// Indicates that the account belongs to a pausable mint
    PausableAccount,

    /// Test variable-length mint extension
    #[cfg(test)]
    VariableLenMintTest = u16::MAX - 2,
    /// Padding extension used to make an account exactly Multisig::LEN, used
    /// for testing
    #[cfg(test)]
    AccountPaddingTest,
    /// Padding extension used to make a mint exactly Multisig::LEN, used for
    /// testing
    #[cfg(test)]
    MintPaddingTest,
}

/// Encapsulates immutable base state data (mint or account) with possible
/// extensions
#[derive(Debug, PartialEq)]
pub struct StateWithExtensions<'data, S: BaseState> {
    /// Unpacked base data
    pub base: S,
    /// Slice of data containing all TLV data, deserialized on demand
    tlv_data: &'data [u8],
}

fn check_account_type<S: BaseState>(account_type: AccountType) -> Result<(), ProgramError> {
    if account_type != S::ACCOUNT_TYPE {
        Err(ProgramError::InvalidAccountData)
    } else {
        Ok(())
    }
}

fn check_min_len_and_not_multisig(input: &[u8], minimum_len: usize) -> Result<(), ProgramError> {
    if input.len() == Multisig::LEN || input.len() < minimum_len {
        Err(ProgramError::InvalidAccountData)
    } else {
        Ok(())
    }
}

impl<'data, S: BaseState + kani::Arbitrary> StateWithExtensions<'data, S> {
    pub fn unpack(input: &'data SparseSlice<u8>) -> Result<Self, ProgramError> {
        // check_min_len_and_not_multisig(input, S::SIZE_OF)?;
        // let (base_data, rest) = input.split_at(S::SIZE_OF);
        //
        // TODO: do we need to do real serialization here?
        let base = S::any();

        // let tlv_data = unpack_tlv_data::<S>(rest)?;
        Ok(Self { base, tlv_data: unsafe { std::slice::from_raw_parts(std::ptr::null(), 0) } })
    }

    pub fn get_extension<V: kani::Arbitrary>(&self) -> Result<&V, ProgramError> {
        Ok(Box::leak(Box::new(kani::any())))
    }

    pub fn get_extension_types(&self) -> Result<FastVec<ExtensionType>, ProgramError> {
        Ok(kani::any())
    }
}

/// Trait for base states, specifying the associated enum
pub trait BaseState {
    /// Associated extension type enum, checked at the start of TLV entries
    const ACCOUNT_TYPE: AccountType;
    // const SIZE_OF: usize;
}

impl BaseState for Account {
    const ACCOUNT_TYPE: AccountType = AccountType::Account;
}

impl BaseState for Mint {
    const ACCOUNT_TYPE: AccountType = AccountType::Mint;
}

/// Trait for base state with extension
pub trait BaseStateWithExtensions<S: BaseState> {
    // /// Get the buffer containing all extension data
    // fn get_tlv_data(&self) -> &[u8];

    // /// Fetch the bytes for a TLV entry
    // fn get_extension_bytes<V: Extension>(&self) -> Result<&[u8], ProgramError> {
    //     get_extension_bytes::<S, V>(self.get_tlv_data())
    // }

    // /// Unpack a portion of the TLV data as the desired type
    // fn get_extension<V: Extension + bytemuck::Pod>(&self) -> Result<&V, ProgramError> {
    //     pod_from_bytes::<V>(self.get_extension_bytes::<V>()?)
    // }

    // /// Unpacks a portion of the TLV data as the desired variable-length type
    // fn get_variable_len_extension<V: Extension + VariableLenPack>(
    //     &self,
    // ) -> Result<V, ProgramError> {
    //     let data = get_extension_bytes::<S, V>(self.get_tlv_data())?;
    //     V::unpack_from_slice(data)
    // }

    // /// Iterates through the TLV entries, returning only the types
    // fn get_extension_types(&self) -> Result<Vec<ExtensionType>, ProgramError> {
    //     get_tlv_data_info(self.get_tlv_data()).map(|x| x.extension_types)
    // }

    // /// Get just the first extension type, useful to track mixed initialization
    // fn get_first_extension_type(&self) -> Result<Option<ExtensionType>, ProgramError> {
    //     get_first_extension_type(self.get_tlv_data())
    // }

    // /// Get the total number of bytes used by TLV entries and the base type
    // fn try_get_account_len(&self) -> Result<usize, ProgramError> {
    //     let tlv_info = get_tlv_data_info(self.get_tlv_data())?;
    //     if tlv_info.extension_types.is_empty() {
    //         Ok(S::SIZE_OF)
    //     } else {
    //         let total_len = tlv_info
    //             .used_len
    //             .saturating_add(BASE_ACCOUNT_AND_TYPE_LENGTH);
    //         Ok(adjust_len_for_multisig(total_len))
    //     }
    // }
    // /// Calculate the new expected size if the state allocates the given
    // /// fixed-length extension instance.
    // /// If the state already has the extension, the resulting account length
    // /// will be unchanged.
    // fn try_get_new_account_len<V: Extension + bytemuck::Pod>(&self) -> Result<usize, ProgramError> {
    //     try_get_new_account_len_for_extension_len::<S, V>(
    //         self.get_tlv_data(),
    //         pod_get_packed_len::<V>(),
    //     )
    // }

    // /// Calculate the new expected size if the state allocates the given
    // /// variable-length extension instance.
    // fn try_get_new_account_len_for_variable_len_extension<V: Extension + VariableLenPack>(
    //     &self,
    //     new_extension: &V,
    // ) -> Result<usize, ProgramError> {
    //     try_get_new_account_len_for_extension_len::<S, V>(
    //         self.get_tlv_data(),
    //         new_extension.get_packed_len()?,
    //     )
    // }
}

#[repr(transparent)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, PartialEq, Debug, Default, kani::Arbitrary)]
pub struct OptionalNonZeroPubkey(pub Pubkey);

impl From<OptionalNonZeroPubkey> for Option<Pubkey> {
    fn from(x: OptionalNonZeroPubkey) -> Option<Pubkey> {
        if x.0.t.iter().all(|x| *x == 0) {
            None
        } else {
            Some(x.0)
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, PartialEq, Debug, Default, kani::Arbitrary)]
pub struct PodU64(pub [u8; 8]);

#[repr(transparent)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, PartialEq, Debug, Default, kani::Arbitrary)]
pub struct PodU16(pub [u8; 2]);

#[repr(transparent)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, PartialEq, Debug, Default, kani::Arbitrary)]
pub struct PodBool(pub u8);

impl From<u64> for PodU64 {
    fn from(x: u64) -> PodU64 {
        Self(x.to_le_bytes())
    }
}

impl From<u16> for PodU16 {
    fn from(x: u16) -> PodU16 {
        Self(x.to_le_bytes())
    }
}

impl From<PodU16> for u16 {
    fn from(x: PodU16) -> u16 {
        u16::from_le_bytes(x.0)
    }
}

impl From<PodU64> for u64 {
    fn from(x: PodU64) -> u64 {
        u64::from_le_bytes(x.0)
    }
}

impl From<bool> for PodBool {
    fn from(b: bool) -> Self {
        Self(b as u8)
    }
}

impl From<PodBool> for bool {
    fn from(b: PodBool) -> Self {
        b.0 != 0
    }
}


pub mod transfer_fee {
    use crate::spl_token_2022::extension::OptionalNonZeroPubkey;
    use crate::spl_token_2022::extension::PodU64;
    use crate::spl_token_2022::extension::PodU16;

    #[repr(C)]
    #[derive(Copy, Clone, PartialEq, Debug, bytemuck::Zeroable, bytemuck::Pod, kani::Arbitrary, Default)]
    pub struct TransferFee {
        pub epoch: PodU64,
        pub maximum_fee: PodU64,
        pub transfer_fee_basis_points: PodU16,
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable, kani::Arbitrary)]
    pub struct TransferFeeConfig {
        /// Optional authority to set the fee
        pub transfer_fee_config_authority: OptionalNonZeroPubkey,
        /// Withdraw from mint instructions must be signed by this key
        pub withdraw_withheld_authority: OptionalNonZeroPubkey,
        /// Withheld transfer fee tokens that have been moved to the mint for
        /// withdrawal
        pub withheld_amount: PodU64,
        /// Older transfer fee, used if `current epoch < new_transfer_fee.epoch`
        pub older_transfer_fee: TransferFee,
        /// Newer transfer fee, used if `current epoch >= new_transfer_fee.epoch`
        pub newer_transfer_fee: TransferFee,
    }
}

pub mod transfer_hook {
    use crate::spl_token_2022::extension::OptionalNonZeroPubkey;

    /// Transfer hook extension data for mints.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable, kani::Arbitrary)]
    pub struct TransferHook {
        /// Authority that can set the transfer hook program id
        pub authority: OptionalNonZeroPubkey,
        /// Program that authorizes the transfer
        pub program_id: OptionalNonZeroPubkey,
    }
}

pub mod confidential_transfer {
    use crate::spl_token_2022::extension::OptionalNonZeroPubkey;
    use crate::spl_token_2022::extension::PodBool;
    use crate::spl_token_2022::extension::PodU64;

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable, kani::Arbitrary)]
    pub struct ConfidentialTransferMint {
        /// Authority to modify the `ConfidentialTransferMint` configuration and to
        /// approve new accounts (if `auto_approve_new_accounts` is true)
        ///
        /// The legacy Token Multisig account is not supported as the authority
        pub authority: OptionalNonZeroPubkey,

        /// Indicate if newly configured accounts must be approved by the
        /// `authority` before they may be used by the user.
        ///
        /// * If `true`, no approval is required and new accounts may be used
        ///   immediately
        /// * If `false`, the authority must approve newly configured accounts (see
        ///   `ConfidentialTransferInstruction::ConfigureAccount`)
        pub auto_approve_new_accounts: PodBool,

        // /// Authority to decode any transfer amount in a confidential transfer.
        // pub auditor_elgamal_pubkey: OptionalNonZeroElGamalPubkey,
    }

    // I don't think contracts can really inspect this value, so we just put a placeholder type

    #[derive(Copy, Clone, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable, kani::Arbitrary)]
    #[repr(C)]
    pub struct CryptBalance {
        placeholder: [u8; 1]
    }

    pub type EncryptedBalance = CryptBalance;
    pub type DecryptableBalance = CryptBalance;

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable, kani::Arbitrary)]
    pub struct ConfidentialTransferAccount {
        /// `true` if this account has been approved for use. All confidential
        /// transfer operations for the account will fail until approval is
        /// granted.
        pub approved: PodBool,

        /// The public key associated with ElGamal encryption
        // pub elgamal_pubkey: PodElGamalPubkey,

        /// The low 16 bits of the pending balance (encrypted by `elgamal_pubkey`)
        pub pending_balance_lo: EncryptedBalance,

        /// The high 48 bits of the pending balance (encrypted by `elgamal_pubkey`)
        pub pending_balance_hi: EncryptedBalance,

        /// The available balance (encrypted by `encryption_pubkey`)
        pub available_balance: EncryptedBalance,

        /// The decryptable available balance
        pub decryptable_available_balance: DecryptableBalance,

        /// If `false`, the extended account rejects any incoming confidential
        /// transfers
        pub allow_confidential_credits: PodBool,

        /// If `false`, the base account rejects any incoming transfers
        pub allow_non_confidential_credits: PodBool,

        /// The total number of `Deposit` and `Transfer` instructions that have
        /// credited `pending_balance`
        pub pending_balance_credit_counter: PodU64,

        /// The maximum number of `Deposit` and `Transfer` instructions that can
        /// credit `pending_balance` before the `ApplyPendingBalance`
        /// instruction is executed
        pub maximum_pending_balance_credit_counter: PodU64,

        /// The `expected_pending_balance_credit_counter` value that was included in
        /// the last `ApplyPendingBalance` instruction
        pub expected_pending_balance_credit_counter: PodU64,

        /// The actual `pending_balance_credit_counter` when the last
        /// `ApplyPendingBalance` instruction was executed
        pub actual_pending_balance_credit_counter: PodU64,
    }
}
