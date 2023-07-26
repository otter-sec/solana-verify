use crate::program_error::ProgramError;
use crate::vec::sparse::*;

/// Check if a program account state is initialized
pub trait IsInitialized {
    /// Is initialized
    fn is_initialized(&self) -> bool;
}

/// Implementors must have a known size
pub trait Sealed: Sized {}

// osec trait
// TODO: we can write a macro that generates this using a global
pub trait Verify {
    fn get_next() -> Self;
    fn expect_unpack(x: Self);
    fn repack(x: Self);
    fn get_packed(i: usize) -> Self;
    fn num_used() -> usize;
}

// non-kani version (so you can build without...)
#[cfg(not(any(kani, feature = "kani")))]
pub trait Pack: Sealed {
    /// The length, in bytes, of the packed representation
    const LEN: usize;
    #[doc(hidden)]
    fn pack_into_slice(&self, dst: &mut [u8]);

    // TODO(ahaberlandt): What prevents someone from calling this?
    #[doc(hidden)]
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError>;

    /// Get the packed length
    fn get_packed_len() -> usize {
        Self::LEN
    }

    /// Unpack from slice and check if initialized
    fn unpack(input: &SparseSlice<u8>) -> Result<Self, ProgramError>
    where
        Self: IsInitialized,
    {
        let value = Self::unpack_unchecked(input)?;
        if value.is_initialized() {
            Ok(value)
        } else {
            Err(ProgramError::UninitializedAccount)
        }
    }

    /// Unpack from slice without checking if initialized
    fn unpack_unchecked(input: &SparseSlice<u8>) -> Result<Self, ProgramError> {
        if input.len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        unimplemented!()
    }

    /// Pack into slice
    fn pack(_src: Self, _dst: &mut SparseSlice<u8>) -> Result<(), ProgramError> {
        unimplemented!()
    }
}

#[cfg(any(kani, feature = "kani"))]
pub trait Pack: Sealed + Verify {
    /// The length, in bytes, of the packed representation
    const LEN: usize;
    #[doc(hidden)]
    fn pack_into_slice(&self, dst: &mut [u8]);

    // TODO(ahaberlandt): What prevents someone from calling this?
    #[doc(hidden)]
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError>;

    /// Get the packed length
    fn get_packed_len() -> usize {
        Self::LEN
    }

    /// Unpack from slice and check if initialized
    fn unpack(input: &SparseSlice<u8>) -> Result<Self, ProgramError>
    where
        Self: IsInitialized,
    {
        let value = Self::unpack_unchecked(input)?;
        if value.is_initialized() {
            Ok(value)
        } else {
            Err(ProgramError::UninitializedAccount)
        }
    }

    /// Unpack from slice without checking if initialized
    fn unpack_unchecked(_input: &SparseSlice<u8>) -> Result<Self, ProgramError> {
        // if input.len() != Self::LEN {
        //     return Err(ProgramError::InvalidAccountData);
        // }
        // Lets not worry about all the different errors, and just return the
        // one that can actually be produced
        // let b: bool = kani::any();
        // if b {

        // XXX: This is a temporary fix because we are currently
        // just asserting that the right number of accounts
        // will be deserialized at the end.
        Ok(Self::get_next())
        // } else {
        //     Err(ProgramError::InvalidAccountData)
        // }
        // Self::unpack_from_slice(input)
    }

    /// Pack into slice
    fn pack(src: Self, _dst: &mut SparseSlice<u8>) -> Result<(), ProgramError> {
        // TODO: We should make sure all the accounts get re-serialized
        // TODO: Need to handle cases where accounts are repacked in
        // a different order
        Self::repack(src);
        Ok(())
    }
}
