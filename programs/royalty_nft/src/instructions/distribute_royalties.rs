use crate::errors;
use crate::state::{RoyaltyPool, UserRoyaltyInfo};
use anchor_lang::prelude::*;

pub fn distribute_royalties(ctx: Context<DistributeRoyalties>, total_royalties: u64) -> Result<()> {
    let royalty_pool = &mut ctx.accounts.royalty_pool_account;
    let user_royalty_info = &mut ctx.accounts.user_royalty_info;

    let current_epoch = royalty_pool.current_epoch;

    for epoch in 0..current_epoch {
        if user_royalty_info
            .claimed_royalties
            .get(epoch as usize)
            .unwrap_or(&0)
            > &0
        {
            continue; //user already claimed royalties for this epoch
        }

        let epoch_royalties = royalty_pool
            .royalties_per_epoch
            .get(epoch as usize)
            .unwrap();
        let user_share =
            (user_royalty_info.shares * epoch_royalties) / royalty_pool.total_royalties;

        user_royalty_info.add_royalty(user_share); //increases the pending royalties for the user

        if epoch as usize >= user_royalty_info.claimed_royalties.len() {
            user_royalty_info.claimed_royalties.push(user_share); //store claimed royalties for this epoch
        } else {
            user_royalty_info.claimed_royalties[epoch as usize] = user_share; //mark the current epoch as claimed
        }
    }

    // Transfer SOL to the user's account
    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        anchor_lang::system_program::Transfer {
            from: royalty_pool.to_account_info(),
            to: ctx.accounts.user.to_account_info(),
        },
    );
    anchor_lang::system_program::transfer(cpi_ctx, user_royalty_info.pending_royalties)?;

    // Reduce the SOL balance in the royalty pool
    royalty_pool.sol_balance -= user_royalty_info.pending_royalties;

    Ok(())
}

#[derive(Accounts)]
pub struct DistributeRoyalties<'info> {
    #[account(
        mut,
        seeds = [RoyaltyPool::SEED_PREFIX, mint.key().as_ref()],  // Derive from Mint
        bump,
    )]
    pub royalty_pool_account: Account<'info, RoyaltyPool>,
    #[account( mut,
         seeds = [b"user_royalty", user.key().as_ref(), mint.key().as_ref()],
          bump)]
    pub user_royalty_info: Account<'info, UserRoyaltyInfo>,
    #[account(mut, signer)]
    pub user: Signer<'info>, // User initiating the claim
    
    ///CHECK : this is safe because we dont read or write from this account
    pub mint: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
