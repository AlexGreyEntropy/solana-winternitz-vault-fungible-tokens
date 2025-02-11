use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};
use crate::state::TokenVault;

#[derive(Accounts)]
pub struct OpenTokenVault<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub token_mint: Account<'info, Mint>,
    
    #[account(
        init,
        payer = payer,
        space = TokenVault::LEN,
        seeds = [b"token-vault", pubkey_root.as_ref()],
        bump
    )]
    pub vault: Account<'info, TokenVault>,
    
    #[account(
        init,
        payer = payer,
        token::mint = token_mint,
        token::authority = vault,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_open_token_vault(
    ctx: Context<OpenTokenVault>,
    pubkey_root: [u8; 32],
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    
    vault.pubkey_root = pubkey_root;
    vault.token_mint = ctx.accounts.token_mint.key();
    vault.token_account = ctx.accounts.vault_token_account.key();
    vault.bump = *ctx.bumps.get("vault").unwrap();
    
    Ok(())
}
