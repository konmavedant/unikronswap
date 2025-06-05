import { Keypair, PublicKey, Connection, Transaction, SystemProgram, VersionedTransaction } from "@solana/web3.js";
import { Program, AnchorProvider, BN, Idl, Wallet } from "@coral-xyz/anchor";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { createHash } from "crypto";
import idlJson from "./idl/unikron.json";

// Replace with your deployed Program ID
const programId = new PublicKey("7saCDPbxRTGEPeyTYZgXyzNVr5LXFPEnYKpVyAqT2QNd");
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

const wallet = Keypair.generate();
const dummyNodeWallet: NodeWallet = {
  payer: wallet,
  publicKey: wallet.publicKey,
  signTransaction: async <T extends Transaction | VersionedTransaction>(tx: T): Promise<T> => {
    if (tx instanceof Transaction) tx.partialSign(wallet);
    return tx;
  },
  signAllTransactions: async <T extends Transaction | VersionedTransaction>(txs: T[]): Promise<T[]> => {
    return txs.map(tx => {
      if (tx instanceof Transaction) tx.partialSign(wallet);
      return tx;
    });
  }
};

const provider = new AnchorProvider(connection, dummyNodeWallet, {});
const idl = idlJson as unknown as Idl;
const program = new Program(idl, programId, provider);

async function run() {
  const swapData = JSON.stringify({
    fromMint: "So11111111111111111111111111111111111111112",
    toMint: "USDC111111111111111111111111111111111111111",
    amountIn: "1000000",
    minOut: "990000",
    route: "via Jupiter"
  });

  const intentHash = createHash("sha256").update(swapData).digest();
  const [swapAccount] = PublicKey.findProgramAddressSync([
    Buffer.from("swap"),
    wallet.publicKey.toBuffer(),
    intentHash
  ], programId);

  console.log("Committing intent...");
  await program.methods.commitSwap(Array.from(intentHash))
    .accounts({
      swap: swapAccount,
      user: wallet.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([wallet])
    .rpc();

  await new Promise(res => setTimeout(res, 2000));

  const amountIn = 1_000_000;
  const routeHash = createHash("sha256").update("jupiter-v3").digest();
  const bounty = wallet.publicKey;
  const treasury = wallet.publicKey;
  const stakers = wallet.publicKey;

  console.log("Revealing intent...");
  await program.methods.revealAndExecute(
    Array.from(Buffer.from(swapData)),
    Array.from(routeHash),
    new BN(amountIn),
  )
    .accounts({
      swap: swapAccount,
      user: wallet.publicKey,
      bounty,
      treasury,
      stakers,
    })
    .signers([wallet])
    .rpc();

  console.log("Swap revealed and executed successfully.");
}

run().catch(err => console.error("Execution error:", err));