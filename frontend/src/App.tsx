import { useState } from 'react';
import { Activity, Loader2, User } from 'lucide-react';
import { WalletProvider, useWallet } from './contexts/WalletContext';
import { ToastProvider } from './contexts/ToastContext';
import { PermissionManagement } from './components/PermissionManagement';
import { PatientHistory } from './components/PatientHistory';
import { DoctorPatientView } from './components/DoctorPatientView';
import { CreateRecord } from './components/CreateRecord';

type Tab = 'permissions' | 'history' | 'doctor' | 'upload';

const AppContent = () => {
  const [activeTab, setActiveTab] = useState<Tab>('permissions');
  const { isConnected, accountId, connectWallet, disconnectWallet } =
    useWallet();
  const [connecting, setConnecting] = useState(false);

  const handleConnect = async () => {
    setConnecting(true);
    connectWallet();
    setTimeout(() => setConnecting(false), 1000);
  };

  const tabs: { id: Tab; label: string }[] = [
    { id: 'permissions', label: 'Permissões' },
    { id: 'history', label: 'Meu Histórico' },
    { id: 'doctor', label: 'Busca Médica' },
    { id: 'upload', label: 'Criar Registro' },
  ];

  return (
    <div className='min-h-screen bg-gradient-to-br from-teal-50 via-blue-50 to-white'>
      <header className='bg-white/80 backdrop-blur-lg border-b border-gray-200 sticky top-0 z-40 shadow-sm'>
        <div className='container mx-auto px-6 py-4'>
          <div className='flex items-center justify-between'>
            <div className='flex items-center gap-3'>
              <div className='p-2 bg-gradient-to-br from-teal-500 to-blue-600 rounded-xl'>
                <Activity className='w-7 h-7 text-white' />
              </div>
              <div>
                <h1 className='text-2xl font-bold bg-gradient-to-r from-teal-600 to-blue-600 bg-clip-text text-transparent'>
                  HealthChain
                </h1>
                <p className='text-xs text-gray-500'>
                  Medical Blockchain on Polkadot
                </p>
              </div>
            </div>

            <div>
              {isConnected ? (
                <div className='flex items-center gap-3'>
                  <div className='bg-green-50 border border-green-200 rounded-xl px-4 py-2 flex items-center gap-2'>
                    <div className='w-2 h-2 bg-green-500 rounded-full animate-pulse' />
                    <span className='text-sm font-medium text-gray-700'>
                      {accountId?.slice(0, 6)}...{accountId?.slice(-4)}
                    </span>
                  </div>
                  <button
                    onClick={disconnectWallet}
                    className='px-4 py-2 text-sm text-gray-600 hover:text-gray-800 transition-colors'
                  >
                    Disconnect
                  </button>
                  <div className='w-10 h-10 bg-gradient-to-br from-teal-400 to-blue-500 rounded-full flex items-center justify-center'>
                    <User className='w-5 h-5 text-white' />
                  </div>
                </div>
              ) : (
                <button
                  onClick={handleConnect}
                  disabled={connecting}
                  className='bg-gradient-to-r from-teal-500 to-blue-600 text-white px-6 py-2.5 rounded-xl font-semibold hover:shadow-lg transition-all disabled:opacity-50 flex items-center gap-2'
                >
                  {connecting ? (
                    <>
                      <Loader2 className='w-4 h-4 animate-spin' />
                      Connecting...
                    </>
                  ) : (
                    'Connect Wallet'
                  )}
                </button>
              )}
            </div>
          </div>
        </div>
      </header>

      <nav className='container mx-auto px-6 pt-8'>
        <div className='bg-white/60 backdrop-blur-md rounded-2xl p-2 shadow-lg border border-gray-200 inline-flex gap-2'>
          {tabs.map(tab => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className={`px-6 py-3 rounded-xl font-medium transition-all ${
                activeTab === tab.id
                  ? 'bg-gradient-to-r from-teal-500 to-blue-600 text-white shadow-md'
                  : 'text-gray-600 hover:text-gray-900 hover:bg-white/50'
              }`}
            >
              {tab.label}
            </button>
          ))}
        </div>
      </nav>

      <main className='container mx-auto px-6 py-8'>
        {activeTab === 'permissions' && <PermissionManagement />}
        {activeTab === 'history' && <PatientHistory />}
        {activeTab === 'doctor' && <DoctorPatientView />}
        {activeTab === 'upload' && <CreateRecord />}
      </main>

      <footer className='container mx-auto px-6 py-8 text-center text-gray-500 text-sm'>
        <p>HealthChain POC - Secure Medical Records on Polkadot Blockchain</p>
      </footer>
    </div>
  );
};

function App() {
  return (
    <WalletProvider>
      <ToastProvider>
        <AppContent />
      </ToastProvider>
    </WalletProvider>
  );
}

export default App;
