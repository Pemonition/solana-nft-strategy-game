import { WalletProvider } from '@solana/wallet-adapter-react';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-wallets';

const wallets = [new PhantomWalletAdapter()];

export const WalletContext: React.FC = ({ children }) => {
  return (
    <WalletProvider wallets={wallets} autoConnect>
      {children}
    </WalletProvider>
  );
};
