use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ContractState {
    pub bump: u8,
    pub total_items: u64,
}

impl ContractState {
    pub const SEED_PREFIX: &'static [u8] = b"deauthe";

    pub fn add_items(&mut self, items: u64) {
        self.total_items += items;
    }

    pub fn add_item(&mut self) {
        self.total_items += 1;
    }
}
