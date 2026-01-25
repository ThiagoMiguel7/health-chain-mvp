import React, { useState, JSX } from 'react';
import { Lock, Loader2 } from 'lucide-react';

import { useToast } from '../contexts/ToastContext';
import { grantAccess, revokeAccess } from '../utils/polkadot.api';
import { Input } from './Input';

function Header(): JSX.Element {
  return (
    <div className='flex items-center gap-3 mb-6'>
      <div className='p-3 bg-gradient-to-br from-teal-500 to-blue-600 rounded-xl'>
        <Lock className='w-6 h-6 text-white' />
      </div>
      <h2 className='text-2xl font-bold text-gray-800'>
        Conceder e Revogar Acesso
      </h2>
    </div>
  );
}

type Actions = 'grant' | 'revoke';

export function PermissionManagement(): JSX.Element {
  const { showToast } = useToast();

  const [doctorId, setDoctorId] = useState('');
  const [loading, setLoading] = useState(false);
  const [patientId, setPatientId] = useState('');
  const [action, setAction] = useState<Actions>('grant');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!doctorId || !patientId) {
      showToast('error', 'Por favor, preencha todos os campos');
      return;
    }

    setLoading(true);
    showToast('info', 'Assinando transação...');

    try {
      let txHash: string = '';
      let blockNumber: number = 0;

      if (action === 'grant') {
        const result = await grantAccess({
          patientAddress: patientId,
          doctorAddress: doctorId,
        });

        if (!result.success) {
          throw result.error;
        }

        txHash = result.transactionHash;
        blockNumber = result.blockNumber;
      } else {
        const result = await revokeAccess({
          patientAddress: patientId,
          doctorAddress: doctorId,
        });

        if (!result.success) {
          throw result.error;
        }

        txHash = result.transactionHash;
        blockNumber = result.blockNumber;
      }

      showToast(
        'success',
        `Permissão ${action === 'grant' ? 'concedida' : 'revogada'} com sucesso! TxHash: ${txHash.slice(0, 10)}... | Bloco: ${blockNumber}`,
      );
      setDoctorId('');
      setPatientId('');
    } catch (err) {
      console.log('Error signing transaction:', err);
      showToast('error', 'Transação falhou. Por favor, tente novamente.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className='max-w-2xl mx-auto'>
      <div className='bg-white/70 backdrop-blur-md rounded-2xl shadow-xl p-8 border border-teal-100'>
        <Header />

        <form onSubmit={handleSubmit} className='space-y-6'>
          <Input
            value={doctorId}
            onChange={setDoctorId}
            title='ID da Conta do Médico'
          />

          <Input
            value={patientId}
            onChange={setPatientId}
            title='ID da Conta do Paciente'
          />

          <div>
            <label className='block text-sm font-medium text-gray-700 mb-2'>
              Ação
            </label>

            <select
              value={action}
              onChange={e => setAction(e.target.value as Actions)}
              className='w-full px-4 py-3 bg-white border border-gray-300 rounded-xl focus:ring-2 focus:ring-teal-500 focus:border-transparent transition-all cursor-pointer'
            >
              <option value='grant'>Conceder Acesso</option>
              <option value='revoke'>Revogar Acesso</option>
            </select>
          </div>

          <button
            type='submit'
            disabled={loading}
            className='w-full bg-gradient-to-r from-teal-500 to-blue-600 text-white py-4 rounded-xl font-semibold hover:shadow-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2'
          >
            {loading ? (
              <>
                <Loader2 className='w-5 h-5 animate-spin' />
                Processando...
              </>
            ) : (
              'Enviar Transação'
            )}
          </button>
        </form>
      </div>
    </div>
  );
}
