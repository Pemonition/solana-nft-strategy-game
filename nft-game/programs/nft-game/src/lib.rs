use anchor_lang::prelude::*;

declare_id!("AVKiu6QFWwMMKqhukX2Tn6LhTFeZWpfv2kW7ErYxU1kn");

#[program]
pub mod nft_game {
    use super::*;

    /// Mint a new NFT and associate metadata with the player account.
    pub fn mint_nft(ctx: Context<MintNFT>, metadata: String) -> Result<()> {
        require!(metadata.len() <= 200, GameError::MetadataTooLong); // Limit metadata length.
        
        let player = &mut ctx.accounts.player;
        player.nft_metadata = metadata;
        Ok(())
    }

    /// List an NFT for sale on the marketplace.
    pub fn list_nft(ctx: Context<ListNFT>, price: u64) -> Result<()> {
        require!(price > 0, GameError::InvalidPrice); // Ensure price is positive.

        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.price = price;
        marketplace.seller = *ctx.accounts.seller.key;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(init, payer = user, space = Player::LEN)]
    pub player: Account<'info, Player>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Player {
    pub nft_metadata: String,
}

impl Player {
    // Calculate account size dynamically.
    const LEN: usize = 8 + 4 + 200; // Discriminator + metadata string length + metadata bytes.
}

#[derive(Accounts)]
pub struct ListNFT<'info> {
    #[account(init, payer = seller, space = Marketplace::LEN)]
    pub marketplace: Account<'info, Marketplace>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Marketplace {
    pub seller: Pubkey,
    pub price: u64,
}

impl Marketplace {
    // Calculate account size dynamically.
    const LEN: usize = 8 + 32 + 8; // Discriminator + seller pubkey + price.
}

#[error_code]
pub enum GameError {
    #[msg("The provided metadata is too long.")]
    MetadataTooLong,
    #[msg("The price must be greater than zero.")]
    InvalidPrice,
}
