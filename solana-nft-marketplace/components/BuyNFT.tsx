const BuyNFT = ({ listing }: { listing: { price: number; seller: string; mint: string } }) => {
    const { publicKey, sendTransaction } = useWallet();
  
    const handleBuyNFT = async () => {
      if (!publicKey) {
        alert('Connect your wallet');
        return;
      }
  
      try {
        const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
        const transaction = new Transaction();
  
        // Add instructions for buy_nft here
  
        const signature = await sendTransaction(transaction, connection);
        console.log('Transaction sent:', signature);
      } catch (error) {
        console.error('Error:', error);
      }
    };
  
    return (
      <div className="p-6 max-w-md mx-auto bg-white rounded-xl shadow-md space-y-4">
        <h2 className="text-lg font-semibold text-gray-900">Buy NFT</h2>
        <p>Mint: {listing.mint}</p>
        <p>Price: {listing.price} SOL</p>
        <button
          onClick={handleBuyNFT}
          className="w-full bg-green-500 text-white py-2 px-4 rounded"
        >
          Buy NFT
        </button>
      </div>
    );
  };
  
  export default BuyNFT;
  