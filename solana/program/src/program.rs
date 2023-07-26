use super::{account_info::AccountInfo, instruction::Instruction};

#[cfg(not(feature = "verify"))]
pub type ProgramResult = Result<(), crate::error::Error>;

#[cfg(feature = "verify")]
pub type ProgramResult = Result<(), crate::program_error::ProgramError>;

pub fn invoke(instruction: &Instruction, account_infos: &[AccountInfo]) -> ProgramResult {
    invoke_signed(instruction, account_infos, &[])
}

pub fn invoke_signed(
    _instruction: &Instruction,
    _account_infos: &[AccountInfo],
    _signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    panic!("Invocation is not supported yet.");

    // Check that the account RefCells are consistent with the request
    // for account_meta in instruction.accounts.iter() {
    //     for account_info in account_infos.iter() {
    //         if account_meta.pubkey == *account_info.key {
    //             if account_meta.is_writable {
    //                 let _ = account_info.try_borrow_mut_lamports()?;
    //                 let _ = account_info.try_borrow_mut_data()?;
    //             } else {
    //                 let _ = account_info.try_borrow_lamports()?;
    //                 let _ = account_info.try_borrow_data()?;
    //             }
    //             break;
    //         }
    //     }
    // }
    // Ok(())
}

pub fn set_return_data(_data: &[u8]) {
    panic!("not implemented");
}
