// Instrucionnes para cuñar NFT
import {
    Connection,
    PublicKey,
    Transaction,
    SystemProgram,
    Keypair,
  } from "@solana/web3.js";
  import { AnchorProvider, Program, Wallet } from "@project-serum/anchor";
  import idl from "./nft_game.json"; // Your Anchor-generated IDL
  
  const connection = new Connection("http://127.0.0.1:8899");
  const wallet = new Wallet(...);
  const provider = new AnchorProvider(connection, wallet, {});
  
  const programId = new PublicKey("AVKiu6QFWwMMKqhukX2Tn6LhTFeZWpfv2kW7ErYxU1kn");
  const program = new Program(idl, programId, provider);
  
  // Call the `mint_nft` function
  const tx = await program.methods
    .mintNft("https://your-metadata-url.json")
    .accounts({
      player: playerKeypair.publicKey,
      user: wallet.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([playerKeypair])
    .rpc();
  
  console.log("Transaction signature:", tx);
  