use crate::account_info::AccountInfo;
use crate::program_error::ProgramError;

pub mod rent {
    use super::Sysvar;
    use crate::pubkey::Pubkey;
    pub use crate::rent::Rent;

    impl Sysvar for Rent {}
    pub fn id() -> Pubkey {
        Pubkey { t: [43] }
    }
}

// note: this is different than Sysvar struct from anchor
pub trait Sysvar: Sized {
    // Provided methods
    // fn size_of() -> usize;

    #[cfg(not(any(kani, feature = "kani")))]
    fn from_account_info(_account_info: &AccountInfo<'_>) -> Result<Self, ProgramError> {
        panic!("not impl")
    }

    #[cfg(any(kani, feature = "kani"))]
    fn from_account_info(_account_info: &AccountInfo<'_>) -> Result<Self, ProgramError>
    where
        Self: kani::Arbitrary,
    {
        Self::get()
    }

    fn to_account_info(&self, _account_info: &mut AccountInfo<'_>) -> Option<()> {
        panic!("not impemented")
    }

    #[cfg(any(kani, feature = "kani"))]
    fn get() -> Result<Self, ProgramError>
    where
        Self: kani::Arbitrary,
    {
        // TODO(ahaberlandt): id check is performed here
        // XXX: Maybe we should reason about failures? prob not useful
        Ok(kani::any())
    }

    #[cfg(not(any(kani, feature = "kani")))]
    fn get() -> Result<Self, ProgramError> {
        unimplemented!()
    }
}
