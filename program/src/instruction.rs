use super::pubkey::Pubkey;


pub const TRANSACTION_LEVEL_STACK_HEIGHT: usize = 1;

#[derive(Default)]
pub struct Instruction {
    /// Pubkey of the instruction processor that executes this instruction
    pub program_id: Pubkey,
    /// Metadata for what accounts should be passed to the instruction processor
    pub accounts: crate::vec::fast::Vec<AccountMeta>,
    /// Opaque data passed to the instruction processor
    pub data: crate::vec::sparse::Vec<u8>,
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub struct AccountMeta {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

impl AccountMeta {
    pub fn new(pubkey: Pubkey, is_signer: bool) -> Self {
        Self {
            pubkey,
            is_signer,
            is_writable: true,
        }
    }

    pub fn new_readonly(pubkey: Pubkey, is_signer: bool) -> Self {
        Self {
            pubkey,
            is_signer,
            is_writable: false,
        }
    }
}

/// Return a fixed stack height for testing
pub fn get_stack_height() -> usize {
    999
}
