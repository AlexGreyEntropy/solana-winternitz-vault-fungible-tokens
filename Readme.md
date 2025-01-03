# Solana Winternitz Vault
A quantum-resistant lamports vault secured by Winternitz One Time Signatures. It utilizes a truncated Keccak256 lattice hash, giving it 224 bits of security, and holds up quite robustly against Grover's algorithm.

# Usage

There are 3 main instructions:

1. Open Vault
2. Split Vault
3. Close Vault

### 1. Open Vault
A vault is opened by generating a new Winternitz keypair, Keccak256 hashing its public key, and committing it as the seed of a PDA of the Winternitz Vault program. This key can be generated randomly, or deterministically from your own private seed and a derivation path. We use Keccak because, unfortunately, we must truncate our hashes algorithm to 224 bits, as 256 bit hashes blow out the instruction data limit by 37 bytes with a CU budget request.