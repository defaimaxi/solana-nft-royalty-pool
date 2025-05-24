use anchor_lang::prelude::*;
use instructions::*;
use mpl_token_metadata::types::Creator;
pub mod errors;
pub mod instructions;
pub mod state;
declare_id!("9FyDLMcnijDNmKwSwhZZCF7Gdp7ibz7LBmpQy9fGZq92");

#[program]
mod royalty_nft {
    use super::*;

    pub fn intialize_contract(ctx: Context<InitializeContract>) -> Result<()> {
        initialize_contract::intialize_contract(ctx)
    }

    pub fn create_nft(
        ctx: Context<CreateNft>,
        symbol: String,
        name: String,
        uri: String,
    ) -> Result<()> {
        instructions::create_nft(ctx, name, symbol, uri)
    }

    pub fn mint_nft(
        ctx: Context<MintNft>,
        symbol: String,
        name: String,
        nft_uri: String,
    ) -> Result<()> {
        instructions::mint_nft(ctx, symbol, name, nft_uri)
    }

    // 2. Buy shares of the NFT
    pub fn buy_shares(ctx: Context<BuyShares>, share_amount: u64) -> Result<()> {
        instructions::buy_shares(ctx, share_amount)
    }

    // 3. Distribute royalties
    pub fn distribute_royalties(
        ctx: Context<DistributeRoyalties>,
        total_royalties: u64,
    ) -> Result<()> {
        instructions::distribute_royalties(ctx, total_royalties)
    }

    pub fn add_royalties_to_pool(ctx: Context<AddRoyaltiesToPool>, amount: u64) -> Result<()> {
        instructions::add_royalties_to_pool(ctx, amount)
    }
}
