import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaWinternitzVault } from "../target/types/solana_winternitz_vault";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  createAccount,
  mintTo,
} from "@solana/spl-token";
import { assert } from "chai";

describe("token-vault", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .SolanaWinternitzVault as Program<SolanaWinternitzVault>;

  let mint: anchor.web3.PublicKey;
  let tokenAccount: anchor.web3.PublicKey;
  
  before(async () => {
    // Create mint and token account for testing
    mint = await createMint(
      provider.connection,
      provider.wallet.payer,
      provider.wallet.publicKey,
      null,
      9
    );

    tokenAccount = await createAccount(
      provider.connection,
      provider.wallet.payer,
      mint,
      provider.wallet.publicKey
    );
  });

  it("Creates a token vault", async () => {
    
  });

  it("Splits tokens with valid signature", async () => {
   
  });
});
