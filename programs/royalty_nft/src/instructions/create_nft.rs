use std::fs::Metadata;

use crate::state::{ContractState, RoyaltyPool};
use anchor_lang::prelude::*;
use anchor_spl::{mint, token};
use anchor_spl::token::{InitializeMint2, Mint, MintTo, Token, TokenAccount};
use mpl_token_metadata::accounts::MasterEdition;
use mpl_token_metadata::instructions::{
    CreateMasterEditionV3Cpi, CreateMasterEditionV3CpiAccounts, CreateMasterEditionV3InstructionArgs, CreateMetadataAccountV3Cpi, CreateV1CpiBuilder
};
use mpl_token_metadata::programs::MPL_TOKEN_METADATA_ID;
use mpl_token_metadata::types::{Creator, PrintSupply, TokenStandard};


pub fn create_nft(
    ctx: Context<CreateNft>,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()> {
    let payer = &ctx.accounts.payer;
    let rent = &ctx.accounts.rent.to_account_info();
    let token_metadata_prgram = &ctx.accounts.token_metadata_program;

    let creators = vec![Creator {
        address: ctx.accounts.payer.key(),
        verified: true,
        share: 100, // Full ownership to the payer
    }];

    // 1. Initialize the mint account for the NFT (1 decimal, fixed supply of 1)

    // token::initialize_mint2(
    //     ctx.accounts.initialize_mint_ctx(),
    //     0, // Decimals: NFTs have 0 decimals
    //     &payer.key(),
    //     Some(&payer.key()),
    // )?;

    //the above is already being done by anchor in the account declaration  

  
    // 2. Create metadata for the NFT
    let mint_key = &ctx.accounts.mint.key();
    let metadata_seeds = &[
        b"metadata",
        mpl_token_metadata::ID.as_ref(),
        &mint_key.to_bytes(),
    ];
    let metadata_pda_from_contract = mpl_token_metadata::accounts::Metadata::find_pda(mint_key);
    let (metadata_pda, _) = Pubkey::find_program_address(metadata_seeds, &mpl_token_metadata::ID);
    msg!("metadata pda : {} and {} and {}", &ctx.accounts.metadata.key(), metadata_pda, metadata_pda_from_contract.0);

    let edition_key_from_contract =mpl_token_metadata::accounts::MasterEdition::find_pda(mint_key);
    msg!("edition key : {} and {}", edition_key_from_contract.0, &ctx.accounts.edition_account.key());

 

    //createV1 is a helper function that can initialize the Mint Account and create the Metadata Account.
    //  If the mint exists already it will only create the metadata Account. 
    //  If you are looking on how to use createMetadataAccountV3 you should be using this function instead.

    let _metadata_creation_result = CreateV1CpiBuilder::new(&token_metadata_prgram.to_account_info())
    .metadata(&ctx.accounts.metadata.to_account_info())
    .mint(&ctx.accounts.mint.to_account_info(), true)
    .authority(&payer)
    .payer(&payer)
    .update_authority(&payer, false)
    .master_edition(Some(&ctx.accounts.edition_account.to_account_info()))
    .system_program(&ctx.accounts.system_program.to_account_info())
    .spl_token_program(Some(&ctx.accounts.token_program.to_account_info()))
    .token_standard(TokenStandard::NonFungible)
    .name(String::from(name))
    .uri(uri)
    .symbol(symbol)
    .seller_fee_basis_points(550)
    .token_standard(TokenStandard::NonFungible)
    .print_supply(PrintSupply::Zero)
    .authority(&payer)
    .invoke();

    msg!("metadata account created");

      //3. create master edition account

      let _master_edition = CreateMasterEditionV3Cpi::new(
        &ctx.accounts.token_metadata_program.to_account_info(),
    CreateMasterEditionV3CpiAccounts{
        edition: &ctx.accounts.edition_account.to_account_info(),
        metadata: &ctx.accounts.metadata.to_account_info(),
        mint: &ctx.accounts.mint.to_account_info(),
        mint_authority: &ctx.accounts.payer.to_account_info(),
        payer: &ctx.accounts.payer.to_account_info(),
        rent: Some(rent),
        system_program: &ctx.accounts.system_program.to_account_info(),
        token_program: &ctx.accounts.token_program.to_account_info(),
        update_authority: &ctx.accounts.payer.to_account_info(),
    }, CreateMasterEditionV3InstructionArgs {
        max_supply: Some(0)
    }).invoke();
    
    msg!("master edition created");

    let master_edition = MasterEdition::find_pda(&ctx.accounts.mint.key());
    msg!("master edition account : {}", master_edition.0);


    //4. mint the nft to the user's account
    let mint_to_res = token::mint_to(ctx.accounts.mint_to_ctx(), 1);
    match mint_to_res {
        Ok(_) => msg!("minting to account succesful"),
        Err(e) => msg!("minting to account failed with error : {}", e),
    }

    

    //5. update contract state
    let contract_state = &mut ctx.accounts.contract_state;
    contract_state.add_item();

    Ok(())
}

#[derive(Accounts)]
pub struct CreateNft<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    ///CHECK : this is safe because we dont read or write from this account
    #[account(
    init,
    seeds = [b"mint"], // Replace with your seed
    bump,
    payer = payer,
    mint::decimals = 0,
    mint::authority = mint,
    mint::freeze_authority = payer,
)]
    pub mint: Account<'info, Mint>, //space is not required when creating spl accounts

    ///CHECK : this is safe because we dont read or write from this account
    #[account(
        mut,
        address=mpl_token_metadata::accounts::Metadata::find_pda(&mint.key()).0
)]
    pub metadata: UncheckedAccount<'info>, //check if a struct has to be created for this

    ///CHECK : this is safe because we dont read or write from this account
    #[account(
    mut,
    address=mpl_token_metadata::accounts::MasterEdition::find_pda(&mint.key()).0
    )]
    pub edition_account: UncheckedAccount<'info>,

    ///CHECK : this is safe because we dont read or write from this account
    #[account(
        init,
        seeds = [payer.key().as_ref(), mint.key().as_ref()],
        bump,
        payer = payer,
        token::mint = mint,
        token::authority = payer,
    )]
    pub token_account: Box<Account<'info, TokenAccount>>,

    ///CHECK : this is safe because we dont read or write from this account
    #[account(
        init,
        payer = payer,
        seeds = [RoyaltyPool::SEED_PREFIX, mint.key().as_ref()],
        bump,
        space = 8 + RoyaltyPool::INIT_SPACE,
    )]
    pub royalty_pool_account: Box<Account<'info, RoyaltyPool>>,

    #[account(mut, seeds = [ContractState::SEED_PREFIX.as_ref()], bump)]
    pub contract_state: Account<'info, ContractState>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    ///CHECK : this is safe because we dont read or write from this account
    #[account(address = MPL_TOKEN_METADATA_ID)]
    pub token_metadata_program: AccountInfo<'info>,
}

impl<'info> CreateNft<'info> {

    fn _initialize_mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, InitializeMint2<'info>> {
        //TODO: this account below might have to be changed later
        let _signer : &[&[&[u8]]] = &[&[b"mint"]];

        CpiContext::new( //should I use new_with_signer here?
            self.token_program.to_account_info(),
            InitializeMint2 {
                mint: self.mint.to_account_info(),
            },
        )
    }

    fn mint_to_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.mint.to_account_info(),
                to: self.token_account.to_account_info(),
                authority: self.payer.to_account_info(),
            },
        )
    }

   
}
