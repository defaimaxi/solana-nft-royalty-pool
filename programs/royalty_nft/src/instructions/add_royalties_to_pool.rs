use anchor_lang::prelude::*;

use crate::state::RoyaltyPool;

pub fn add_royalties_to_pool(ctx: Context<AddRoyaltiesToPool>, amount: u64) -> Result<()> {
    let royalty_pool = &mut ctx.accounts.royalty_pool_account;

    // Transfer SOL to the RoyaltyPool PDA
    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.admin.to_account_info(),
            to: royalty_pool.to_account_info(),
        },
    );
    anchor_lang::system_program::transfer(cpi_ctx, amount)?;

    // Start a new distribution epoch
    royalty_pool.current_epoch += 1;

    // Add new royalties to the current epoch
    royalty_pool.royalties_per_epoch.push(amount);

    // Update the total royalties and SOL balance in the pool
    royalty_pool.total_royalties += amount;
    royalty_pool.sol_balance += amount;

    Ok(())
}

#[derive(Accounts)]
pub struct AddRoyaltiesToPool<'info> {
    #[account(
        mut,
        seeds = [RoyaltyPool::SEED_PREFIX, mint.key().as_ref()],  // Derive from Mint
        bump,
    )]
    pub royalty_pool_account: Account<'info, RoyaltyPool>,
    ///CHECK : this is safe because we dont read or write from this account
    pub mint: AccountInfo<'info>,
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
