import { useState, JSX } from 'react';
import { FileText, Loader2, ExternalLink } from 'lucide-react';

import { useToast } from '../contexts/ToastContext';
import { useWallet } from '../contexts/WalletContext';
import { checkAccess, readOwnData } from '../utils/polkadot.api';
import { openFile } from '../utils/ipfs-functions';
import { Input } from './Input';

function Header(): JSX.Element {
  return (
    <div className='flex items-center gap-3 mb-6'>
      <div className='p-3 bg-gradient-to-br from-blue-500 to-teal-600 rounded-xl'>
        <FileText className='w-6 h-6 text-white' />
      </div>
      <h2 className='text-2xl font-bold text-gray-800'>
        Recuperação de Histórico Próprio
      </h2>
    </div>
  );
}

export function PatientHistory(): JSX.Element {
  const { showToast } = useToast();
  const { accountId: walletAccountId } = useWallet();

  const [cid, setCid] = useState('');
  const [loading, setLoading] = useState(false);
  const [accountId, setAccountId] = useState('');

  const handleView = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!accountId || !cid) {
      showToast('error', 'Por favor, preencha todos os campos');
      return;
    }

    setLoading(true);
    showToast('info', 'Verificando registro na blockchain...');

    const hasAccess = await checkAccess({
      patientAddress: accountId,
      doctorAddress: walletAccountId || accountId,
    });

    if (!hasAccess) {
      showToast('error', 'Você não tem acesso a este histórico');
      setLoading(false);
      return;
    }

    try {
      const result = await readOwnData({
        fileHashHex: cid,
        patientAddress: accountId,
      }).catch(err => err);

      if (!result.success) {
        showToast('error', 'Registro não encontrado na blockchain');
        setLoading(false);
        return;
      }

      showToast('success', 'Registro verificado! Abrindo gateway do IPFS...');
      await openFile(cid);
    } catch (err) {
      console.log('Error verifying record:', err);
      showToast('error', 'Verificação falhou. Por favor, tente novamente.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className='max-w-2xl mx-auto'>
      <div className='bg-white/70 backdrop-blur-md rounded-2xl shadow-xl p-8 border border-blue-100'>
        <Header />

        <form onSubmit={handleView} className='space-y-6'>
          <Input
            value={accountId}
            onChange={setAccountId}
            title=' Meu ID de Conta'
            placeholder={walletAccountId || '5Grw...'}
          >
            {walletAccountId && (
              <button
                type='button'
                onClick={() => setAccountId(walletAccountId)}
                className='mt-2 text-sm text-blue-600 hover:text-blue-700 font-medium'
              >
                Usar carteira conectada: {walletAccountId.slice(0, 8)}...
              </button>
            )}
          </Input>

          <Input
            value={cid}
            onChange={setCid}
            placeholder='Qm...'
            title='Hash do Arquivo / CID'
          />

          <button
            type='submit'
            disabled={loading}
            className='w-full bg-gradient-to-r from-blue-500 to-teal-600 text-white py-4 rounded-xl font-semibold hover:shadow-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2'
          >
            {loading ? (
              <>
                <Loader2 className='w-5 h-5 animate-spin' />
                Verificando...
              </>
            ) : (
              <>
                <ExternalLink className='w-5 h-5' />
                Buscar & Visualizar
              </>
            )}
          </button>
        </form>
      </div>
    </div>
  );
}
