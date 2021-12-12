use anchor_lang::prelude::*;
// use anchor_spl::token::accessor::authority;
// use anchor_spl::associated_token::AssociatedToken;
// use anchor_spl::token::{self, set_authority, Mint, SetAuthority, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const DECIMAL_MOVEMENT: f64 = 10_000_000_000_000.0;

fn get_slope(start_price: i64, start_time: i64, reserve_price: Option<i64>, end_time: i64) -> i64 {
    println!("end: {}, start: {}", end_time, start_time);
    let top = reserve_price.or(Some(0)).unwrap() - start_price;
    println!("top {}", top);
    let bottom = end_time - start_time;
    ((top as f64 / bottom as f64) * DECIMAL_MOVEMENT).round() as i64
}

fn get_y_intercept(start_price: i64, start_time: i64, slope: i64) -> i64 {
    let slopef64 = slope as f64 / DECIMAL_MOVEMENT;
    let slope_start_time = slopef64 * start_time as f64;
    println!("slope * start time: {}", slope_start_time);

    ((start_price as f64 - slope_start_time) * DECIMAL_MOVEMENT).round() as i64
}

fn get_current_price_as_i64(current_time: i64, slope: i64, y_intercept: i64) -> i64 {
    let slopef64 = slope as f64 / DECIMAL_MOVEMENT;
    let y_interf64 = y_intercept as f64 / DECIMAL_MOVEMENT;
    let res = slopef64 * current_time as f64;
    println!("the res: {}", res);
    ((res + y_interf64) * DECIMAL_MOVEMENT).round() as i64
}

fn get_lamports_from_sol(cur_sol: i64) -> u64 {
    let cur_solf64 = cur_sol as f64 / DECIMAL_MOVEMENT;
    (cur_solf64 * 1_000_000_000.0).round() as u64
}

#[program]
pub mod dutch_auction {
    use super::*;
    pub fn initialize(
        ctx: Context<Create>,
        authority: Pubkey,
        // auction values
        start_time: i64,
        end_time: i64,
        start_price: i64,
        reserve_price: Option<i64>,
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
        // NOTES FOR PROD
        // - This is not proper escrow software
        // - Currently the purchasing account is just paying to end the auction. Transferring ownership of a some token or whatnot should be trivial.

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
            if current_timestamp > auction.end_time {
                auction.is_ended = true;
                msg!("auction is ended");
                Ok(())
            } else {
                // attempt all fund transfers and then end the auction
                let current_price_sol =
                    get_current_price_as_i64(current_timestamp, auction.slope, auction.y_intercept);

                let current_price_lamps = get_lamports_from_sol(current_price_sol);

                anchor_lang::solana_program::program::invoke(
                    &anchor_lang::solana_program::system_instruction::transfer(
                        purchaser.to_account_info().key,
                        authority.to_account_info().key,
                        current_price_lamps,
                    ),
                    &[
                        purchaser.to_account_info(),
                        authority.to_account_info(),
                        ctx.accounts.system_program.to_account_info(),
                    ],
                )?;

                //end the auction
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
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Auction {
    pub authority: Pubkey,
    // timestamps (should be positive)
    pub start_time: i64,
    pub end_time: i64,
    // prices (should be positive)
    pub start_price: i64,
    pub reserve_price: Option<i64>,
    // math values (should only be positive)
    pub slope: i64,
    pub y_intercept: i64,
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
