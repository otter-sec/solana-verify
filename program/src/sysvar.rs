use crate::account_info::AccountInfo;
use crate::program_error::ProgramError;
use crate::pubkey::Pubkey;

pub mod rent {
    use super::Sysvar;
    use crate::pubkey::Pubkey;
    pub use crate::rent::Rent;

    impl Sysvar for Rent {}
    pub fn id() -> Pubkey {
        Pubkey::new_from_array([43])
    }
}


pub mod instructions {
    use crate::{account_info::AccountInfo, instruction::Instruction, program_error::ProgramError, pubkey::Pubkey};

    pub struct Instructions;

    impl Instructions {
        pub fn id() -> Pubkey {
            Pubkey::new_from_array([44])
        }
    }

    // Note: Fix this later as needed
    pub fn load_current_index_checked(
        _: &AccountInfo,
    ) -> Result<u16, ProgramError> {
        Ok(0)
    }

    pub fn load_instruction_at_checked(
        _: usize,
        _: &AccountInfo,
    ) -> Result<Instruction, ProgramError> {
        Ok(Instruction::default())
    }
}

pub trait SysvarId {
    /// The `Pubkey` of the sysvar.
    fn id() -> Pubkey;

    /// Returns `true` if the given pubkey is the program ID.
    fn check_id(pubkey: &Pubkey) -> bool;
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
