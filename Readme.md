# Solana Winternitz Vault
A quantum-resistant lamports vault secured by Winternitz One Time Signatures.

# Usage
A vault is initialized by generating a new Winternitz keypair. This can be generated randomly, or deterministically from your own hidden seed. The pubkey is then hashed with keccak256 and used as the sole seed to a PDA address. We use Keccak because unfortunately, using the full 256 bit hash blows our our instruction data by 37 bytes with a CU request instruction. As such, Keccak is more resistant to length-extension attacks than Sha256, it is the obvious choice.