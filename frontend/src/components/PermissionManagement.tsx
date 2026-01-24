import React, { useState, JSX } from 'react';
import { Lock, Loader2 } from 'lucide-react';

import { useToast } from '../contexts/ToastContext';
import { mockPermissionTransaction } from '../utils/blockchain';
import { grantAccess } from '../utils/polkadot.api';

type InputProps = {
  title: string;
  value: string;
  placeholder: string;
  onChange: (value: string) => void;
};

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

function Input({
  title,
  value,
  placeholder,
  onChange,
}: Readonly<InputProps>): JSX.Element {
  const handleOnChange = ({
    target,
  }: React.ChangeEvent<HTMLInputElement>): void => onChange(target.value);

  return (
    <div>
      <label className='block text-sm font-medium text-gray-700 mb-2'>
        {title}
      </label>

      <input
        type='text'
        value={value}
        placeholder={placeholder}
        onChange={handleOnChange}
        className='w-full px-4 py-3 bg-white border border-gray-300 rounded-xl focus:ring-2 focus:ring-teal-500 focus:border-transparent transition-all'
      />
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
      showToast('error', 'Please fill in all fields');
      return;
    }

    setLoading(true);
    showToast('info', 'Signing transaction...');

    try {
      await grantAccess({ patientAddress: patientId, doctorAddress: doctorId });

      const { txHash, blockNumber } = await mockPermissionTransaction(
        doctorId,
        patientId,
        action,
      );
      showToast(
        'success',
        `Permission ${action === 'grant' ? 'granted' : 'revoked'} successfully! TxHash: ${txHash.slice(0, 10)}... | Block: ${blockNumber}`,
      );
      setDoctorId('');
      setPatientId('');
    } catch (err) {
      console.log('Error signing transaction:', err);
      showToast('error', 'Transaction failed. Please try again.');
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
            placeholder='5Grw...'
            onChange={setDoctorId}
            title='ID da Conta do Médico'
          />

          <Input
            value={patientId}
            placeholder='5Grw...'
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
              'Submit Transaction'
            )}
          </button>
        </form>
      </div>
    </div>
  );
}
