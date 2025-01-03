pub mod instructions;
pub mod tests;
use instructions::*;

use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

// wntrRTssxbf1rz9RPJ4xNBbpXxfsidQJT177NN3MXhB
pub const ID: Pubkey = [
    0x0e, 0x09, 0x41, 0x96, 0x88, 0xf6, 0x50, 0xcd, 0xb0, 0x5d, 0x81, 0x14, 0x81, 0xe4, 0x9a, 0x03,
    0x4a, 0xd6, 0xb9, 0x89, 0x00, 0x31, 0x23, 0x65, 0x90, 0xb0, 0xef, 0x63, 0x98, 0x19, 0x46, 0xf2,
];

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match accounts.len() {
        3 => OpenVault::process(instruction_data, accounts),
        2 => CloseVault::process(instruction_data, accounts),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
