use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct TokenVault {
    pub pubkey_root: [u8; 32],
    pub token_mint: Pubkey,
    pub token_account: Pubkey,
    pub bump: u8,
}

impl TokenVault {
    pub const LEN: usize = 8 + // discriminator
        32 + // pubkey_root
        32 + // token_mint
        32 + // token_account
        1;   // bump
}
