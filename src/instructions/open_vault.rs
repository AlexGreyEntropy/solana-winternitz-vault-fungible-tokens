use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccount;

pub struct OpenVault;

impl OpenVault {
    pub fn process(instruction_data: &[u8], accounts: &[AccountInfo]) -> ProgramResult {
        // Assert we have exactly 2 accounts
        let [payer, vault, _system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // Split Bump from WinternitzPubkey hash
        let (bump, data) = instruction_data
            .split_first_chunk::<1>()
            .ok_or(ProgramError::InvalidInstructionData)?;

        // Get our pubkey hash from the instruction data
        let pubkey_hash: [u8; 32] = data
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        let lamports = Rent::get()?.minimum_balance(0);
        let seeds = [Seed::from(&pubkey_hash), Seed::from(bump)];
        let signers = [Signer::from(&seeds)];

        // Create our vault
        CreateAccount {
            from: payer,
            to: vault,
            lamports,
            space: 0,
            owner: &crate::ID,
        }
        .invoke_signed(&signers)
    }
}
