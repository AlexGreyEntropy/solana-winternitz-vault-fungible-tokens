use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::TokenVault;
use crate::utils::winternitz::{verify_winternitz_signature, verify_pubkey_root};

#[derive(Accounts)]
pub struct SplitTokenVault<'info> {
    #[account(
        mut,
        seeds = [b"token-vault", vault.pubkey_root.as_ref()],
        bump = vault.bump,
        close = refund_account
    )]
    pub vault: Account<'info, TokenVault>,
    
    #[account(
        mut,
        constraint = vault_token_account.key() == vault.token_account
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub split_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub refund_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: Refund wallet for rent
    pub refund_account: AccountInfo<'info>,
    
    pub token_program: Program<'info, Token>,
}

pub fn handle_split_token_vault(
    ctx: Context<SplitTokenVault>,
    amount: u64,
    signature: Vec<u8>,
    message: Vec<u8>,
) -> Result<()> {
    // Verify Winternitz signature using existing logic
    let recovered_pubkey = verify_winternitz_signature(&signature, &message)?;
    verify_pubkey_root(&recovered_pubkey, &ctx.accounts.vault.pubkey_root)?;
    
    // Transfer specified amount to split account
    let seeds = &[
        b"token-vault",
        ctx.accounts.vault.pubkey_root.as_ref(),
        &[ctx.accounts.vault.bump],
    ];
    let signer = &[&seeds[..]];

    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.split_token_account.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
        signer,
    );
    token::transfer(transfer_ctx, amount)?;
    
    // Transfer remaining balance to refund account
    let remaining = ctx.accounts.vault_token_account.amount - amount;
    if remaining > 0 {
        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_token_account.to_account_info(),
                to: ctx.accounts.refund_token_account.to_account_info(),
                authority: ctx.accounts.vault.to_account_info(),
            },
            signer,
        );
        token::transfer(transfer_ctx, remaining)?;
    }
    
    Ok(())
}
