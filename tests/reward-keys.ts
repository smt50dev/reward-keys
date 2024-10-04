import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RewardKeys } from "../target/types/reward_keys";
import { clusterApiUrl, Connection, Transaction } from "@solana/web3.js";

describe("reward-keys", () => {

  // const providerUrl = "http://127.0.0.1:8899";
  const providerUrl = clusterApiUrl('mainnet-beta');
  const connection = new Connection(providerUrl, "confirmed");

  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as anchor.Wallet;
  anchor.setProvider(provider);

  const program = anchor.workspace.RewardKeys as Program<RewardKeys>;

  // it("Is initialized!", async () => {
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });
  // it("Key Mint!", async () => {
  //   const [counterPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("counter")], program.programId);
  //   console.log("Counter PDA:", counterPDA);
  //   let counter = (await program.account.counter.fetch(counterPDA)).value;
  //   console.log("Start counter:", counter)
  //   const tx = new Transaction();
  //   // const tx = []
  //   function createPDA(initialCount: number, newCount: number) {

  //     const countBuffer = Buffer.alloc(8);
  //     countBuffer.writeBigUInt64LE(BigInt(initialCount + newCount));
  //     return anchor.web3.PublicKey.findProgramAddressSync(
  //       [Buffer.from("key"), countBuffer],
  //       program.programId
  //     )[0];
  //   }
  //   for (let i = 0; i < 5; i++) {

  //     // @ts-ignore
  //     const _tx = await program.methods.createKey().accounts({ rewardKey: createPDA(Number(counter), i) }).instruction()
  //     // tx.push(_tx)
  //     tx.add(_tx);
  //   }

  //   const latestBlockhash = await connection.getLatestBlockhash();
  //   tx.recentBlockhash = latestBlockhash.blockhash;
  //   tx.feePayer = wallet.publicKey;

  //   // @ts-ignore
  //   const signature = await wallet.signTransaction(tx);

  //   const sig = await connection.sendRawTransaction(signature.serialize()).catch(e => {console.log("LOGS", e); return; });
  //   // const tx = await program.methods.createKey()
  //   //   .accounts({ })
  //   //   .rpc();

  //   console.log("Tx signature:", sig);
  //   counter = (await program.account.counter.fetch(counterPDA)).value;
  //   console.log("End counter:", counter)
  // });

  it("PDA TEST", async () => {
    const pdas = await program.account.counter.fetch("EZxpLjCh5MNWczCKyRHicfrRVxgAn4rqQLUCP6ExesAS")
    console.log(pdas);
  })
});
