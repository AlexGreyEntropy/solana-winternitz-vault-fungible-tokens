use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Invalid signature length")]
    InvalidSignatureLength,
    #[msg("Invalid message length")]
    InvalidMessageLength,
    #[msg("Invalid public key")]
    InvalidPublicKey,
    #[msg("Invalid checksum")]
    InvalidChecksum,
    #[msg("Invalid chain code")]
    InvalidChainCode,
    #[msg("Invalid merkle proof")]
    InvalidMerkleProof,
    #[msg("Invalid merkle root")]
    InvalidMerkleRoot,
    #[msg("Invalid signature")]
    InvalidSignature,
    #[msg("Invalid token account owner")]
    InvalidTokenAccountOwner,
    #[msg("Invalid token mint")]
    InvalidTokenMint,
    #[msg("Token account mismatch")]
    TokenAccountMismatch,
    #[msg("Insufficient token balance")]
    InsufficientTokenBalance,
    #[msg("Token account is frozen")]
    TokenAccountFrozen,
    #[msg("Token account is closed")]
    TokenAccountClosed,
    #[msg("Authority mismatch")]
    AuthorityMismatch,
    #[msg("Invalid vault PDA")]
    InvalidVaultPDA,
    #[msg("Vault already initialized")]
    VaultAlreadyInitialized,
    #[msg("Vault not empty")]
    VaultNotEmpty,
    #[msg("Public key does not match root")]
    PublicKeyMismatch,
}
