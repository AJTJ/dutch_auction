import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { DutchAuction } from "../target/types/dutch_auction";
const { SystemProgram } = anchor.web3;
import assert from "assert";
import * as spl from "@solana/spl-token";

describe("dutch_auction", () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.DutchAuction as Program<DutchAuction>;
  const auction = anchor.web3.Keypair.generate();
  const owner = anchor.web3.Keypair.generate();
  const purchaser = anchor.web3.Keypair.generate();
  const purchaserWithoutFunds = anchor.web3.Keypair.generate();
  const price = 1000000000;
  const insufficient_amount = 100000000;
  const more_than_enough = 100000000000;

  // fill the account with lamps
  before(async () => {
    const signature = await program.provider.connection.requestAirdrop(
      purchaserWithoutFunds.publicKey,
      insufficient_amount
    );
    await program.provider.connection.confirmTransaction(signature);
  });
  // fill the account with lamps
  before(async () => {
    const signature = await program.provider.connection.requestAirdrop(
      owner.publicKey,
      more_than_enough
    );
    await program.provider.connection.confirmTransaction(signature);
  });

  // fill the account with lamps
  before(async () => {
    const signature = await program.provider.connection.requestAirdrop(
      purchaser.publicKey,
      more_than_enough
    );
    await program.provider.connection.confirmTransaction(signature);
  });

  it("It initializes the account and creates an auction!", async () => {
    const [mint, mintBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("mint")],
      program.programId
    );

    // Dec 12th, 2021
    let start_time = new anchor.BN(1639341245);
    // January first, 2022
    let end_time = new anchor.BN(1641094445);
    // start price is in LAMPORTS
    let start_price_lamps = new anchor.BN(price);
    // Optional reserve_price
    let reserve_price = null;

    let tx = await program.rpc.initialize(
      mintBump,
      start_time,
      end_time,
      start_price_lamps,
      reserve_price,
      {
        accounts: {
          auction: auction.publicKey,
          authority: owner.publicKey,
          systemProgram: SystemProgram.programId,
          mint: mint,
          tokenProgram: spl.TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        },
        // Needs both since owner and auction are mutable (one for payer, other for init)
        signers: [owner, auction],
      }
    );

    console.log("Transaction: ", tx);
  });

  it("An account without enough funds cannot purchase the item", async () => {
    const [mint, mintBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("mint")],
      program.programId
    );

    let balance_before = await provider.connection.getBalance(
      purchaserWithoutFunds.publicKey
    );
    const account_before = await program.account.auction.fetch(
      auction.publicKey
    );
    assert.ok(account_before.isEnded === false);

    let tx = await program.rpc.claim({
      accounts: {
        auction: auction.publicKey,
        authority: owner.publicKey,
        systemProgram: SystemProgram.programId,
        mint: mint,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        purchaser: purchaserWithoutFunds.publicKey,
      },
      signers: [owner, purchaserWithoutFunds],
    });

    let balance_after = await provider.connection.getBalance(
      purchaserWithoutFunds.publicKey
    );
    const account_after = await program.account.auction.fetch(
      auction.publicKey
    );
    assert.ok(account_after.isEnded === false);

    console.log(
      "balance before: ",
      balance_before,
      "- balance after: ",
      balance_after
    );
  });

  it("The price can be paid, ending the auction", async () => {
    const [mint, mintBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("mint")],
      program.programId
    );
    const account_before = await program.account.auction.fetch(
      auction.publicKey
    );
    assert.ok(account_before.isEnded === false);

    let balance_before = await provider.connection.getBalance(
      purchaser.publicKey
    );

    let tx = await program.rpc.claim({
      accounts: {
        auction: auction.publicKey,
        authority: owner.publicKey,
        systemProgram: SystemProgram.programId,
        mint: mint,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        purchaser: purchaser.publicKey,
      },
      signers: [owner, purchaser],
    });

    let balance_after = await provider.connection.getBalance(
      purchaser.publicKey
    );
    const account_after = await program.account.auction.fetch(
      auction.publicKey
    );
    assert.ok(account_after.isEnded === true);

    console.log(
      "balance before: ",
      balance_before,
      "- balance after: ",
      balance_after
    );
    assert.ok(balance_before > balance_after);

    console.log("Transaction: ", tx);
  });
});
