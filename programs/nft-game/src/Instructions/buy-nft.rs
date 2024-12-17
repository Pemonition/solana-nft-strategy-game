//Comprar NFT
use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer, Token};

#[derive(Accounts)]
pub struct BuyNft<'info> {
    #[account(mut, has_one = seller)]
    pub listing_account: Account<'info, Listing>,
    #[account(mut)]
    pub nft_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn buy_nft(ctx: Context<BuyNft>) -> Result<()> {
    let listing_account = &mut ctx.accounts.listing_account;

    // Transfer SOL to seller
    **ctx.accounts.seller.to_account_info().try_borrow_mut_lamports()? += **ctx.accounts.buyer.to_account_info().lamports.borrow_mut()?;
    **ctx.accounts.buyer.to_account_info().lamports.borrow_mut()? -= listing_account.price;

    // Transfer NFT ownership
    ctx.accounts.nft_account.owner = ctx.accounts.buyer.key();

    // Close listing
    listing_account.close(ctx.accounts.seller.to_account_info())?;

    Ok(())
}
