# Solana Winternitz Vault

A quantum-resistant vault system for Solana, implementing both native SOL and SPL token storage using Winternitz One-Time Signatures (W-OTS). This system provides post-quantum security for your digital assets on Solana.

## 🌟 Features

### Native SOL Vault
- ✨ Quantum-resistant vault creation for SOL
- 🔐 One-time signature-based security
- 🌳 Merkle tree verification
- 💫 Single-use design for maximum security

### Token Vault
- 🪙 Support for all SPL tokens
- 🏦 PDA-controlled token accounts
- 🔄 Secure token transfers
- 🔒 Automatic vault closure
- 🛡️ Post-quantum security

## 🏗️ Architecture

### Vault Types
1. **Native SOL Vault**
   - Direct SOL storage
   - Lamport-based transfers
   - PDA-based vault control

2. **Token Vault**
   - SPL token storage
   - Associated Token Account management
   - PDA authority over tokens

### Security Model
- Winternitz One-Time Signatures (W-OTS)
- Merkle tree authentication
- Single-use vault pattern
- Zero on-chain private key storage

## 📋 Instructions

### Native SOL Vault

'''typescript
// Create vault
open(pubkey_root: [u8; 32])
// Split vault
split(
amount: u64,
signature: Vec<u8>,
message: Vec<u8>
)
'''
### Token Vault

'''typescript
// Create token vault
open_token_vault(pubkey_root: [u8; 32])
// Split token vault
split_token_vault(
amount: u64,
signature: Vec<u8>,
message: Vec<u8>
)
'''

## 💻 Usage Examples

### Native SOL Vault

'''typescript
// Initialize a vault
const tx = await program.methods
.open(pubkeyRoot)
.accounts({
payer: wallet.publicKey,
vault: vaultPDA,
systemProgram: SystemProgram.programId,
})
.rpc();
// Split vault contents
const splitTx = await program.methods
.split(
new BN(amount),
signature,
message
)
.accounts({
vault: vaultPDA,
split: recipientAddress,
refund: refundAddress,
})
.rpc();
'''

### Token Vault

'''typescript
// Create token vault
const tx = await program.methods
.openTokenVault(pubkeyRoot)
.accounts({
payer: wallet.publicKey,
tokenMint: mintAddress,
vault: vaultPDA,
vaultTokenAccount: vaultTokenPDA,
systemProgram: SystemProgram.programId,
tokenProgram: TOKEN_PROGRAM_ID,
rent: SYSVAR_RENT_PUBKEY,
})
.rpc();
// Split token vault
const splitTx = await program.methods
.splitTokenVault(
new BN(amount),
signature,
message
)
.accounts({
vault: vaultPDA,
vaultTokenAccount: vaultTokenPDA,
splitTokenAccount: recipientTokenAccount,
refundTokenAccount: refundTokenAccount,
refundAccount: refundWallet,
tokenProgram: TOKEN_PROGRAM_ID,
})
.rpc();
'''

## 🛠️ Development Setup

### Prerequisites
- Rust 1.68.0 or later
- Solana CLI tools
- Anchor Framework 0.28.0 or later
- Node.js 16+

### Installation

### Clone the repository
git clone https://github.com/yourusername/solana-winternitz-vault.git
cd solana-winternitz-vault

### Install dependencies
yarn install

### Build the program
anchor build

### Run tests
anchor test

### Building

'''bash
anchor build
'''

### Testing

'''bash
anchor test
'''

### Deployment

'''bash
anchor deploy
'''

## 🔒 Security Considerations

1. **Single-Use Principle**
   - Each vault must only be used once
   - Never reuse signatures
   - Always create new vaults for new transactions

2. **Message Security**
   - Use unique messages for each signature
   - Include timestamp or nonce in messages
   - Verify message integrity

3. **Token Safety**
   - Validate token account ownership
   - Verify mint addresses
   - Check account balances

4. **Signature Verification**
   - Always verify signatures
   - Check Merkle proofs
   - Validate public key roots

## 🤝 Contributing

We welcome contributions! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Winternitz One-Time Signature scheme
- Solana blockchain
- Anchor Framework
- SPL Token Program

## 📚 Additional Resources

- [Solana Documentation](https://docs.solana.com/)
- [Anchor Framework Documentation](https://www.anchor-lang.com/)
- [SPL Token Documentation](https://spl.solana.com/token)
- [Post-Quantum Cryptography](https://en.wikipedia.org/wiki/Post-quantum_cryptography)

## ⚠️ Disclaimer

This software is provided "as is", without warranty of any kind. Use at your own risk.

---

Built with ❤️ for the Solana ecosystem
