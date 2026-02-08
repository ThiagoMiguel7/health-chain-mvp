import {
  createContext,
  useContext,
  useState,
  ReactNode,
  useMemo,
  useCallback,
} from 'react';

interface WalletContextType {
  isConnected: boolean;
  accountId: string | null;
  connectWallet: () => void;
  disconnectWallet: () => void;
}

const WalletContext = createContext<WalletContextType>({} as WalletContextType);

// eslint-disable-next-line react-refresh/only-export-components
export const useWallet = (): WalletContextType => {
  const context = useContext(WalletContext);
  if (!context) {
    throw new Error('useWallet must be used within WalletProvider');
  }
  return context;
};

export const WalletProvider = ({ children }: { children: ReactNode }) => {
  const [isConnected, setIsConnected] = useState(false);
  const [accountId, setAccountId] = useState<string | null>(null);

  const connectWallet = useCallback(() => {
    setTimeout(() => {
      setIsConnected(true);
      setAccountId('5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY');
    }, 1000);
  }, []);

  const disconnectWallet = useCallback(() => {
    setIsConnected(false);
    setAccountId(null);
  }, []);

  const value = useMemo(
    () => ({
      isConnected,
      accountId,
      connectWallet,
      disconnectWallet,
    }),
    [isConnected, accountId, connectWallet, disconnectWallet],
  );

  return (
    <WalletContext.Provider value={value}>{children}</WalletContext.Provider>
  );
};
