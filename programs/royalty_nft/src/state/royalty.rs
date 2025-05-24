use anchor_lang::prelude::*;

pub const MAX_EPOCHS: u64 = 50; //TODO: change this to something smaller after figuring out a way to delete data

#[account]
#[derive(InitSpace)]
pub struct UserRoyaltyInfo {
    pub bump: u8,               //to push off curve and retrieve
    pub shares: u64,            // User's NFT share balance
    pub pending_royalties: u64, // Pending royalties to be claimed
    #[max_len(MAX_EPOCHS)]
    pub claimed_royalties: Vec<u64>, // Royalties claimed for each epoch, index is the epoch number, value is the amount
}

#[account]
#[derive(InitSpace)]
pub struct RoyaltyPool {
    //unique to each mint and user combination
    pub last_sale_id: u64,    // Last sale id
    pub total_royalties: u64, // Total royalties ever added to the pool
    pub current_epoch: u64,   // The current royalty distribution epoch
    #[max_len(MAX_EPOCHS)]
    pub royalties_per_epoch: Vec<u64>, // List of royalties distributed per epoch, index is epoch
    pub sol_balance: u64,
}

impl UserRoyaltyInfo {
    pub const SEED_PREFIX: &'static [u8] = b"user_royalty";

    pub fn add_royalty(&mut self, royalty: u64) {
        self.pending_royalties += royalty;
    }

    pub fn add_shares(&mut self, share_amount: u64) -> Result<()> {
        self.shares += share_amount;
        Ok(())
    }
}

impl RoyaltyPool {
    pub const SEED_PREFIX: &'static [u8] = b"royalty_pool";

    fn add_royalties(&mut self, royalties: u64) {
        self.total_royalties += royalties;
        self.royalties_per_epoch.push(royalties);
        self.current_epoch += 1;
    }

    fn get_royalties(&self, epoch: u64) -> u64 {
        self.royalties_per_epoch[epoch as usize]
    }

    fn get_latest_epoch_royalties(&self) -> u64 {
        self.royalties_per_epoch.last().unwrap_or(&0).clone()
    }

    fn get_total_royalties(&self) -> u64 {
        self.total_royalties
    }
}
