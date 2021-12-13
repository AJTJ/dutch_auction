import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { DutchAuction } from "../target/types/dutch_auction";
const { SystemProgram } = anchor.web3;
import assert from "assert";

describe("dutch_auction", () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const providerWallet = provider.wallet;
  const program = anchor.workspace.DutchAuction as Program<DutchAuction>;
  const auction = anchor.web3.Keypair.generate();
  const newUser = anchor.web3.Keypair.generate();

  // fill the account with lamps
  before(async () => {
    const signature = await program.provider.connection.requestAirdrop(
      newUser.publicKey,
      1000000000000
    );
    await program.provider.connection.confirmTransaction(signature);
  });

  it("It initializes the account and creates an auction!", async () => {
    // Dec 12th, 2021
    let start_time = new anchor.BN(1639341245);
    // January first, 2022
    let end_time = new anchor.BN(1641094445);
    // start price is in LAMPORTS
    let start_price_lamps = new anchor.BN(1000);
    // Optional reserve_price
    let reserve_price = null;

    let tx = await program.rpc.initialize(
      providerWallet.publicKey,
      start_time,
      end_time,
      start_price_lamps,
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

    let balance_before = await provider.connection.getBalance(
      newUser.publicKey
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

    let balance_after = await provider.connection.getBalance(newUser.publicKey);
    const account_after = await program.account.auction.fetch(
      auction.publicKey
    );
    assert.ok(account_after.isEnded === true);

    assert.ok(balance_before > balance_after);

    console.log("Transaction: ", tx);
  });
});
