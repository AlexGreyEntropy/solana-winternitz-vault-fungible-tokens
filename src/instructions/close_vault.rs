use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};
use solana_winternitz::signature::WinternitzSignature;

pub struct CloseVault;

impl CloseVault {
    pub fn process(instruction_data: &[u8], accounts: &[AccountInfo]) -> ProgramResult {
        // Assert we have exactly 2 accounts
        let [vault, to] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // Split Bump from WinternitzSignature
        let (bump, data) = instruction_data
            .split_first_chunk::<1>()
            .ok_or(ProgramError::InvalidInstructionData)?;

        // Assert our data has the correct length
        let signature_bytes: [u8; core::mem::size_of::<WinternitzSignature>()] = data
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        // Convert instruction data to WinternitzSignature and recover its pubkey hash
        let pubkey_hash = WinternitzSignature::from(signature_bytes)
            .recover_pubkey(to.key())
            .hash();

        // Fast PDA equivalence check
        let data = [
            pubkey_hash.as_ref(),
            bump,
            crate::ID.as_ref(),
            b"ProgramDerivedAddress",
        ];

        if solana_nostd_sha256::hashv(&data).ne(vault.key()) {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Close Vault and refund balance to Owner
        vault.realloc(0, false)?;

        *to.try_borrow_mut_lamports()? += vault.lamports();
        *vault.try_borrow_mut_lamports()? = 0;

        vault.assign(&Pubkey::default());

        Ok(())
    }
}
