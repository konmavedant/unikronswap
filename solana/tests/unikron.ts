import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Unikron } from "../target/types/unikron";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";

describe("unikron", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Unikron as Program<Unikron>;

  const user = provider.wallet;
  const nonce = new anchor.BN(1);
  const expiry = new anchor.BN(Math.floor(Date.now() / 1000) + 600); // expiry 10 mins from now
  const dummyHash = new Array(32).fill(1); // 32-byte dummy hash

  it("should commit a trade intent", async () => {
    const [intentPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("intent"), user.publicKey.toBuffer(), nonce.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    await program.methods
      .commitTrade(dummyHash as any, nonce, expiry)
      .accounts({
        swapIntent: intentPda,
        user: user.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const intentAccount = await program.account.swapIntent.fetch(intentPda);

    expect(intentAccount.user.toBase58()).to.equal(user.publicKey.toBase58());
    expect(intentAccount.intentHash).to.deep.equal(dummyHash);
    expect(intentAccount.nonce.toString()).to.equal(nonce.toString());
    expect(intentAccount.expiry.toString()).to.equal(expiry.toString());
    expect(intentAccount.revealed).to.be.false;
  });
});
