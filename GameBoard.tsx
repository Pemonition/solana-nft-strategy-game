const GameBoard = () => {
    return (
      <div className="grid grid-cols-8 grid-rows-8">
        {[...Array(64)].map((_, i) => (
          <div key={i} className="w-12 h-12 border border-gray-300"></div>
        ))}
      </div>
    );
  };
  