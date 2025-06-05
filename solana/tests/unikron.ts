// tests/unikron.ts

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Unikron } from "../target/types/unikron";
import { assert } from "chai";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import BN from "bn.js";

const { Keypair } = anchor.web3;

describe("unikron", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Unikron as Program<Unikron>;
  const user = Keypair.generate();
  let intentPda: PublicKey;
  let nonce = new BN(1);

  it("Commits a trade intent", async () => {
    // Fund user
    const airdropSig = await provider.connection.requestAirdrop(
      user.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSig);

    // Derive PDA
    [intentPda] = await PublicKey.findProgramAddress(
      [Buffer.from("intent"), user.publicKey.toBuffer(), Buffer.from(nonce.toArray("le", 8))],
      program.programId
    );

    const intentHashBytes = anchor.utils.bytes.utf8.encode("dummyhash").slice(0, 32);
    const intentHash = Array.from(intentHashBytes);
    const expiry = new BN(Math.floor(Date.now() / 1000) + 600); // expires in 10 minutes

    await program.methods
      .commitTrade(intentHash, nonce, expiry)
      .accounts({
        user: user.publicKey,
        system_program: SystemProgram.programId,
        swap_intent: intentPda,
        })


      .signers([user])
      .rpc();

    const storedIntent = await program.account.swapIntent.fetch(intentPda);
    assert.ok(storedIntent.user.equals(user.publicKey));
    assert.ok(storedIntent.intentHash.every((byte, i) => byte === intentHash[i]));
  });
});
