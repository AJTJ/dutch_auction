import * as anchor from "@project-serum/anchor";
import * as spl from "@solana/spl-token";
import { Program } from "@project-serum/anchor";
import serumCmn from "@project-serum/common";
import { DutchAuction } from "../target/types/dutch_auction";
import { publicKey } from "@project-serum/anchor/dist/cjs/utils";
const { SystemProgram, PublicKey } = anchor.web3;
import assert from "assert";

describe("dutch_auction", () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const providerWallet = provider.wallet;
  const program = anchor.workspace.DutchAuction as Program<DutchAuction>;
  const auction = anchor.web3.Keypair.generate();
  const newUser = anchor.web3.Keypair.generate();

  before(async () => {
    const signature = await program.provider.connection.requestAirdrop(
      newUser.publicKey,
      1000000000000
    );
    await program.provider.connection.confirmTransaction(signature);
  });

  it("It initializes the account and creates an auction!", async () => {
    // Dec 10th, 2021
    let start_time = new anchor.BN(1639341245);
    // January first, 2022
    let end_time = new anchor.BN(1641094445);
    // start price is in LAMPORTS
    let start_price = new anchor.BN(1000);
    // Optional reserve_price
    let reserve_price = null;

    let tx = await program.rpc.initialize(
      providerWallet.publicKey,
      start_time,
      end_time,
      start_price,
      reserve_price,
      {
        accounts: {
          auction: auction.publicKey,
          user: providerWallet.publicKey,
          systemProgram: SystemProgram.programId,
        },
        signers: [auction],
      }
    );

    console.log("Transaction: ", tx);
  });

  it("The price can be paid, ending the auction", async () => {
    const account_before = await program.account.auction.fetch(
      auction.publicKey
    );

    assert.ok(account_before.isEnded === false);
    console.log(
      "balance before",
      await provider.connection.getBalance(newUser.publicKey)
    );

    let tx = await program.rpc.claim({
      accounts: {
        auction: auction.publicKey,
        authority: providerWallet.publicKey,
        purchaser: newUser.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [newUser],
    });
    console.log(
      "balance after",
      await provider.connection.getBalance(newUser.publicKey)
    );
    console.log("Transaction: ", tx);

    const account_after = await program.account.auction.fetch(
      auction.publicKey
    );
    assert.ok(account_after.isEnded === true);
  });
});

// console.log(await provider.connection.getBalance(newUser.publicKey));

// let tx = await program.rpc.claim({
//   accounts: {
//     auction: auction.publicKey,
//     authority: providerWallet.publicKey,
//     purchaser: newUser.publicKey,
//   },
//   signers: [],
// });

// const tx = await program.rpc.initialize(new anchor.BN(1234), {
//   accounts: {
//     myAccount: myAccount.publicKey,
//     user: anchor.Provider.wallet.publicKey,
//     systemProgram: SystemProgram.programId,
//   },
//   signers: [myAccount],
// });

// const tx = await program.rpc.initialize({});
// await program.rpc.initialize(
//   program.provider.wallet.publicKey,
//   start_time,
//   end_time,
//   start_price, null,
//   {
//     accounts: { user: provider.wallet.publicKey,
//       systemProgram: SystemProgram.programId, },
//   },
//   signers: []
// );

// await provider.connection.confirmTransaction(
//   await provider.connection.requestAirdrop(purchaser.publicKey, 2000),
//   "confirmed"
// );

// await provider.confirmTransaction(
//   await provider.connection.requestAirdrop(purchaser.publicKey, 2000),
//   "confirmed"
// );

// let purchaser = await serumCmn.createAccountRentExempt(
//   program.provider,
//   purchaserId.publicKey,
//   300000
// );

// MINT THINGS
// const [mint, mintBump] = await anchor.web3.PublicKey.findProgramAddress(
//   [],
//   program.programId
// );

// console.log({ mint, mintBump });

// let ourAssociatedTokens = await spl.Token.getAssociatedTokenAddress(
//   spl.ASSOCIATED_TOKEN_PROGRAM_ID,
//   spl.TOKEN_PROGRAM_ID,
//   mint,
//   program.provider.wallet.publicKey
// );
// console.log(await provider.connection.getBalance(newUser.publicKey));
// console.log(await provider.connection.getBalance(newUser.publicKey));
``;
// import * as anchor from '@project-serum/anchor';
// import { Program } from '@project-serum/anchor';
// import { DutchAuction } from '../target/types/dutch_auction';

// describe('dutch_auction', () => {

//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.Provider.env());

//   const program = anchor.workspace.DutchAuction as Program<DutchAuction>;

//   it('Is initialized!', async () => {
//     // Add your test here.
//     const tx = await program.rpc.initialize({});
//     console.log("Your transaction signature", tx);
//   });
// });
