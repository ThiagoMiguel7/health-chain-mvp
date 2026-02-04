import { useState, JSX, FormEvent } from 'react';
import { Loader2, FileText, Search } from 'lucide-react';

import { useToast } from '../contexts/ToastContext';
import { useWallet } from '../contexts/WalletContext';

import { Input } from './Input';
import {
  checkAccess,
  PatientHistoryConverted,
  readPatientHistory,
} from '../utils/polkadot.api';

type HistoryProps = { history?: PatientHistoryConverted[] };

function Header(): JSX.Element {
  return (
    <div className='flex items-center gap-3 mb-6'>
      <div className='p-3 bg-gradient-to-br from-blue-500 to-teal-600 rounded-xl'>
        <FileText className='w-6 h-6 text-white' />
      </div>
      <h2 className='text-2xl font-bold text-gray-800'>
        Recuperação todo Histórico
      </h2>
    </div>
  );
}

function HistoryItem({
  url,
  fileHash,
  createdBy,
  createdAt,
}: Readonly<PatientHistoryConverted>): JSX.Element {
  return (
    <div className='bg-white rounded-lg p-4 space-y-6' key={createdAt}>
      <label className='text-xs font-medium text-gray-500 uppercase mb-1 block'>
        Data de criação:
      </label>
      <code className='text-sm text-gray-800 font-mono'>{createdAt}</code>

      <label className='text-xs font-medium text-gray-500 uppercase mb-1 block'>
        Médico:
      </label>
      <code className='text-sm text-gray-800 font-mono'>{createdBy}</code>

      <label className='text-xs font-medium text-gray-500 uppercase mb-1 block'>
        CID do arquivo:
      </label>
      <code className='text-sm text-gray-800 font-mono'>{fileHash}</code>

      <label className='text-xs font-medium text-gray-500 uppercase mb-1 block'>
        <a
          href={url}
          target='_blank'
          rel='noopener noreferrer'
          className='text-blue-600 hover:text-blue-700'
        >
          Abrir registro
        </a>
      </label>
    </div>
  );
}

function HistoryList({ history }: Readonly<HistoryProps>): JSX.Element {
  if (!history) return <></>;

  if (history.length === 0) {
    return (
      <div className='mt-8 bg-gradient-to-br from-blue-50 to-teal-50 border border-blue-200 rounded-xl p-6 space-y-6'>
        <p className='text-gray-500'>Nenhum histórico encontrado</p>
      </div>
    );
  }

  return (
    <div className='mt-8 bg-gradient-to-br from-blue-50 to-teal-50 border border-blue-200 rounded-xl p-6 space-y-6'>
      {history.map(HistoryItem)}
    </div>
  );
}

export function AllHistory(): JSX.Element {
  const { showToast } = useToast();
  const { accountId } = useWallet();

  const [loading, setLoading] = useState(false);
  const [doctorAddress, setDoctorAddress] = useState('');
  const [patientAddress, setPatientAddress] = useState('');
  const [history, setHistory] = useState<PatientHistoryConverted[]>();

  const handleView = async (e: FormEvent): Promise<void> => {
    e.preventDefault();

    if (!patientAddress || !doctorAddress) {
      showToast('error', 'Por favor, preencha todos os campos');
      return;
    }

    setLoading(true);
    showToast('info', 'Verificando registro na blockchain...');

    const hasAccess = await checkAccess({
      doctorAddress,
      patientAddress,
    });

    if (!hasAccess) {
      showToast('error', 'Você não tem acesso a este histórico');
      setLoading(false);
      setHistory(undefined);
      return;
    }

    try {
      const result = await readPatientHistory({
        patientAddress: patientAddress,
      });

      if (!result.length) {
        showToast('info', 'Registros não encontrado na blockchain');
        setLoading(false);
        setHistory([]);
        return;
      }

      showToast('success', 'Registros encontrados!');
      setHistory(result);
    } catch (err) {
      setHistory(undefined);
      console.log('Error verifying record:', err);
      showToast('error', 'Verificação falhou. Por favor, tente novamente.');
    } finally {
      setLoading(false);
    }
  };

  const handlePatientAddressChange = (value: string) => {
    setPatientAddress(value);
    setHistory(undefined);
  };

  const handleDoctorAddressChange = (value: string) => {
    setDoctorAddress(value);
    setHistory(undefined);
  };

  return (
    <div className='max-w-2xl mx-auto'>
      <div className='bg-white/70 backdrop-blur-md rounded-2xl shadow-xl p-8 border border-blue-100'>
        <Header />

        <form onSubmit={handleView} className='space-y-6'>
          <div className='space-y-6'>
            <Input
              value={patientAddress}
              title='Conta do paciente (Dono)'
              onChange={handlePatientAddressChange}
            >
              {accountId && (
                <button
                  type='button'
                  onClick={() => handlePatientAddressChange(accountId)}
                  className='mt-2 text-sm text-blue-600 hover:text-blue-700 font-medium'
                >
                  Usar carteira conectada: {accountId.slice(0, 8)}...
                </button>
              )}
            </Input>

            <Input
              value={doctorAddress}
              onChange={handleDoctorAddressChange}
              title='Conta do Médico (Visualizador)'
            />
          </div>

          <button
            type='submit'
            disabled={loading}
            className='w-full bg-gradient-to-r from-blue-600 to-teal-500 text-white py-4 rounded-xl font-semibold hover:shadow-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2'
          >
            {loading ? (
              <>
                <Loader2 className='w-5 h-5 animate-spin' />
                Buscando registros...
              </>
            ) : (
              <>
                <Search className='w-5 h-5' />
                Buscar histórico
              </>
            )}
          </button>
        </form>

        <HistoryList history={history} />
      </div>
    </div>
  );
}
