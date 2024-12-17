//Procesamiento lógico del contrato
use anchor_lang::prelude::*;

pub fn process_instruction<'info>(
    program_id: &Pubkey,
    accounts: &[AccountInfo<'info>],
    instruction_data: &[u8],
) -> Result<()> {
    let instruction = Instruction::unpack(instruction_data)?;

    match instruction {
        Instruction::MintNft { metadata } => mint_nft(ctx, metadata),
        Instruction::ListNft { price } => list_nft(ctx, price),
        Instruction::BuyNft => buy_nft(ctx),
    }
}
