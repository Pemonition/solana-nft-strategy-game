import { useState } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { PublicKey, Transaction, Connection } from '@solana/web3.js';

const ListNFT = () => {
  const { publicKey, sendTransaction } = useWallet();
  const [nftMint, setNftMint] = useState('');
  const [price, setPrice] = useState('');

  const handleListNFT = async () => {
    if (!publicKey) {
      alert('Connect your wallet');
      return;
    }

    try {
      const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
      const transaction = new Transaction();

      // Add instructions for list_nft here

      const signature = await sendTransaction(transaction, connection);
      console.log('Transaction sent:', signature);
    } catch (error) {
      console.error('Error:', error);
    }
  };

  return (
    <div className="p-6 max-w-md mx-auto bg-white rounded-xl shadow-md space-y-4">
      <h2 className="text-lg font-semibold text-gray-900">List Your NFT</h2>
      <input
        type="text"
        placeholder="NFT Mint Address"
        value={nftMint}
        onChange={(e) => setNftMint(e.target.value)}
        className="w-full p-2 border rounded"
      />
      <input
        type="number"
        placeholder="Price in SOL"
        value={price}
        onChange={(e) => setPrice(e.target.value)}
        className="w-full p-2 border rounded"
      />
      <button
        onClick={handleListNFT}
        className="w-full bg-blue-500 text-white py-2 px-4 rounded"
      >
        List NFT
      </button>
    </div>
  );
};

export default ListNFT;
