use anchor_lang::prelude::*;
// use anchor_spl::token::accessor::authority;
// use anchor_spl::associated_token::AssociatedToken;
// use anchor_spl::token::{self, set_authority, Mint, SetAuthority, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

fn get_slope(start_price: u64, start_time: u64, reserve_price: Option<u64>, end_time: u64) -> u64 {
    (reserve_price.or(Some(0)).unwrap() - start_price) / (end_time - start_time)
}

fn get_y_intercept(start_price: u64, start_time: u64, slope: u64) -> u64 {
    let res = slope as u128 * start_time as u128;
    start_price - res as u64
}

fn get_current_price(current_time: i64, slope: u64, y_intercept: u64) -> u64 {
    let res = slope as u128 * current_time as u128;
    res as u64 + y_intercept
}

#[program]
pub mod dutch_auction {
    use super::*;
    pub fn initialize(
        ctx: Context<Create>,
        authority: Pubkey,
        // auction values
        start_time: u64,
        end_time: u64,
        start_price: u64,
        reserve_price: Option<u64>,
    ) -> ProgramResult {
        let auction = &mut ctx.accounts.auction;
        // auction values
        auction.authority = authority;
        auction.start_time = start_time;
        auction.end_time = end_time;
        auction.start_price = start_price;
        auction.reserve_price = reserve_price;
        auction.is_ended = false;

        let slope = get_slope(start_price, start_time, reserve_price, end_time);
        let y_intercept = get_y_intercept(start_price, start_time, slope);

        auction.slope = slope;
        auction.y_intercept = y_intercept;
        Ok(())
    }

    pub fn claim(ctx: Context<Claim>) -> ProgramResult {
        msg!("yo");
        // NOTES FOR PROD
        // - This is not proper escrow software
        // - Currently the purchasing account is just paying to end the auction. They deserve a prize eventually.

        let auction = &mut ctx.accounts.auction;
        let authority = &mut ctx.accounts.authority;
        let purchaser = &mut ctx.accounts.purchaser;

        if auction.is_ended {
            msg!("auction is ended");
            Ok(())
        } else {
            let clock = Clock::get()?;
            let current_timestamp = clock.unix_timestamp;

            // refuse transaction and end the auction if current time is past the end_time
            if current_timestamp as u64 > auction.end_time {
                auction.is_ended = true;
                msg!("auction is ended");
                Ok(())
            } else {
                // attempt all fund transfers and then end the auction
                let other_price =
                    get_current_price(current_timestamp, auction.slope, auction.y_intercept);

                let current_price = 1000;

                msg!("price1: {}", other_price);
                **purchaser.try_borrow_mut_lamports()? -= current_price;
                msg!("price2: {}", other_price);
                **authority.try_borrow_mut_lamports()? += current_price;

                auction.is_ended = true;
                Ok(())
            }
        }
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 64 + 64)]
    pub auction: Account<'info, Auction>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub auction: Account<'info, Auction>,

    #[account(mut)]
    pub authority: AccountInfo<'info>,

    #[account(mut)]
    pub purchaser: Signer<'info>,
}

#[account]
pub struct Auction {
    pub authority: Pubkey,
    // timestamps (should be positive)
    pub start_time: u64,
    pub end_time: u64,
    // prices (should be positive)
    pub start_price: u64,
    pub reserve_price: Option<u64>,
    // math values (should only be positive)
    pub slope: u64,
    pub y_intercept: u64,
    // other
    pub is_ended: bool,
}

// pub fn join(ctx: Context<Join>) -> Result<()> {
//     let user_list = &ctx.accounts.airdrop.user_list;
//     let max_users = ctx.accounts.airdrop.max_users;

//     if user_list.len() == max_users as usize {
//         return Err(ErrorCode::UserListFull.into());
//     }

//     let user = ctx.accounts.user.to_account_info();
//     let from = &mut ctx.accounts.airdrop_owned_lamports.to_account_info();
//     let airdrop = &mut ctx.accounts.airdrop.to_account_info();
//     **from.try_borrow_mut_lamports()? -= 1_000_000_000;
//     **airdrop.try_borrow_mut_lamports()? += 1_000_000_000;
//     ctx.accounts.airdrop.user_list.push(user.key());
//     Ok(())
// }

// const tempAcct = anchor.web3.Keypair.generate();

//     const programId = new anchor.web3.PublicKey(program.idl.metadata.address);

//     const transaction = new anchor.web3.Transaction().add(
//       anchor.web3.SystemProgram.createAccount({
//         fromPubkey: users[0].publicKey,
//         lamports: 1 * LAMPORTS_PER_SOL,
//         newAccountPubkey: tempAcct.publicKey,
//         programId: programId,
//         space: 256,
//       })
//     );

//     const signature = await anchor.web3.sendAndConfirmTransaction(
//       provider.connection,
//       transaction,
//       [users[0], tempAcct],
//     );

//     await program.rpc.join(
//       {
//         accounts: {
//           user: users[0].publicKey,
//           airdrop: airdrop.publicKey,
//           airdropOwnedLamports: tempAcct.publicKey,
//           systemProgram: anchor.web3.SystemProgram.programId,
//         },
//         signers: [],
//       }
//     );
