import { createContext, useContext, useState, ReactNode } from 'react';

interface WalletContextType {
  isConnected: boolean;
  accountId: string | null;
  connectWallet: () => void;
  disconnectWallet: () => void;
}

const WalletContext = createContext<WalletContextType | undefined>(undefined);

export const useWallet = () => {
  const context = useContext(WalletContext);
  if (!context) {
    throw new Error('useWallet must be used within WalletProvider');
  }
  return context;
};

export const WalletProvider = ({ children }: { children: ReactNode }) => {
  const [isConnected, setIsConnected] = useState(false);
  const [accountId, setAccountId] = useState<string | null>(null);

  const connectWallet = () => {
    setTimeout(() => {
      setIsConnected(true);
      setAccountId('5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY');
    }, 1000);
  };

  const disconnectWallet = () => {
    setIsConnected(false);
    setAccountId(null);
  };

  return (
    <WalletContext.Provider value={{ isConnected, accountId, connectWallet, disconnectWallet }}>
      {children}
    </WalletContext.Provider>
  );
};
