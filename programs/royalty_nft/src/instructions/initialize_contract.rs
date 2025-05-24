use crate::state::ContractState;
use anchor_lang::prelude::*;

pub fn intialize_contract(ctx: Context<InitializeContract>) -> Result<()> {
    let contract_state = &mut ctx.accounts.contract_state;
    contract_state.bump = ctx.bumps.contract_state;
    contract_state.total_items = 0;
    msg!("intialized");
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeContract<'info> {
    #[account(init_if_needed,
    payer = payer,
    space = 8 + ContractState::INIT_SPACE,
    seeds = [ContractState::SEED_PREFIX.as_ref()],
    bump,
    )]
    pub contract_state: Account<'info, ContractState>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
