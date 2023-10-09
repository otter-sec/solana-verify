use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{clock::Slot, pubkey::Pubkey};
use std::borrow::Cow;

/// The maximum number of addresses that a lookup table can hold
pub const LOOKUP_TABLE_MAX_ADDRESSES: usize = 256;

/// The serialized size of lookup table metadata
pub const LOOKUP_TABLE_META_SIZE: usize = 56;

#[cfg(any(kani, feature = "kani"))]
use solana_program::pubkey::kani_new_pubkey;

#[derive(Debug, PartialEq, Eq, Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct LookupTableMeta {
    /// Lookup tables cannot be closed until the deactivation slot is
    /// no longer "recent" (not accessible in the `SlotHashes` sysvar).
    pub deactivation_slot: Slot,
    /// The slot that the table was last extended. Address tables may
    /// only be used to lookup addresses that were extended before
    /// the current bank's slot.
    pub last_extended_slot: Slot,
    /// The start index where the table was last extended from during
    /// the `last_extended_slot`.
    pub last_extended_slot_start_index: u8,
    /// Authority address which must sign for each modification.
    pub authority: Option<Pubkey>,
    // Padding to keep addresses 8-byte aligned
    pub _padding: u16,
    // Raw list of addresses follows this serialized structure in
    // the account's data, starting from `LOOKUP_TABLE_META_SIZE`.
}

#[cfg(any(kani, feature = "kani"))]
impl kani::Arbitrary for LookupTableMeta {
    fn any() -> Self {
        Self {
            deactivation_slot: kani::any::<u64>(),
            last_extended_slot: kani::any::<u64>(),
            last_extended_slot_start_index: kani::any::<u8>(),
            authority: Some(*kani_new_pubkey()),
            _padding: kani::any::<u16>(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, BorshDeserialize, BorshSerialize)]
pub struct AddressLookupTable<'a> {
    pub meta: LookupTableMeta,
    pub addresses: Cow<'a, [Pubkey]>,
}

#[cfg(any(kani, feature = "kani"))]
impl<'a> kani::Arbitrary for AddressLookupTable<'a> {
    fn any() -> Self {
        Self {
            meta: kani::any::<LookupTableMeta>(),
            addresses: Cow::from(vec![*kani_new_pubkey()]),
        }
    }
}
