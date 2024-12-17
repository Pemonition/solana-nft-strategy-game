use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer, Token};

#[derive(Accounts)]
pub struct ListNft<'info> {
    #[account(mut, has_one = authority)]
    pub nft_account: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = 8 + 32 + 8 + 32)]
    pub listing_account: Account<'info, Listing>,
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct Listing {
    pub seller: Pubkey,
    pub price: u64,
    pub nft_mint: Pubkey,
}

pub fn list_nft(ctx: Context<ListNft>, price: u64) -> Result<()> {
    let listing_account = &mut ctx.accounts.listing_account;

    // Validate owner
    if ctx.accounts.nft_account.owner != ctx.accounts.authority.key() {
        return Err(ErrorCode::InvalidOwner.into());
    }

    // Populate listing data
    listing_account.seller = ctx.accounts.authority.key();
    listing_account.price = price;
    listing_account.nft_mint = ctx.accounts.nft_account.mint;

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid NFT owner")]
    InvalidOwner,
}
