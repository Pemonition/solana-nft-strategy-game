const Marketplace = () => {
    const [nfts, setNFTs] = useState([]);
  
    useEffect(() => {
      // Fetch NFTs from blockchain
    }, []);
  
    return (
      <div className="grid grid-cols-2 gap-4">
        {nfts.map((nft, index) => (
          <div key={index} className="border p-4">
            <img src={nft.image} alt={nft.name} />
            <p>{nft.name}</p>
            <button>Buy</button>
          </div>
        ))}
      </div>
    );
  };
  
  