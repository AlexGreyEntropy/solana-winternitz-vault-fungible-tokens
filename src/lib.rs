use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;

declare_id!("WNTZVaLmV6TaLWRKxhJgQL8HYzLwUFb5Bf1MJ5wZBVh");

#[program]
pub mod solana_winternitz_vault {
    use super::*;
    
    pub fn open(ctx: Context<Open>, pubkey_root: [u8; 32]) -> Result<()> {
        instructions::open::handle_open(ctx, pubkey_root)
    }

    pub fn split(
        ctx: Context<Split>,
        amount: u64,
        signature: Vec<u8>,
        message: Vec<u8>,
    ) -> Result<()> {
        instructions::split::handle_split(ctx, amount, signature, message)
    }

    pub fn open_token_vault(
        ctx: Context<OpenTokenVault>,
        pubkey_root: [u8; 32],
    ) -> Result<()> {
        instructions::open_token_vault::handle_open_token_vault(ctx, pubkey_root)
    }

    pub fn split_token_vault(
        ctx: Context<SplitTokenVault>,
        amount: u64,
        signature: Vec<u8>,
        message: Vec<u8>,
    ) -> Result<()> {
        instructions::split_token_vault::handle_split_token_vault(ctx, amount, signature, message)
    }
}
