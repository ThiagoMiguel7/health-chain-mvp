import { useState, JSX } from 'react';
import { Stethoscope, Loader2, ExternalLink, Shield } from 'lucide-react';

import { useToast } from '../contexts/ToastContext';

import { checkAccess, readPatientData } from '../utils/polkadot.api';
import { openFile } from '../utils/ipfs-functions';

import { Input } from './Input';

function Header(): JSX.Element {
  return (
    <div className='flex items-center gap-3 mb-6'>
      <div className='p-3 bg-gradient-to-br from-teal-600 to-blue-500 rounded-xl'>
        <Stethoscope className='w-6 h-6 text-white' />
      </div>

      <h2 className='text-2xl font-bold text-gray-800'>
        Busca de Histórico de Paciente
      </h2>
    </div>
  );
}

export function DoctorPatientView(): JSX.Element {
  const { showToast } = useToast();

  const [cid, setCid] = useState('');
  const [doctorId, setDoctorId] = useState('');
  const [loading, setLoading] = useState(false);
  const [patientId, setPatientId] = useState('');

  const handleVerifyAndView = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!doctorId || !patientId || !cid) {
      showToast('error', 'Por favor, preencha todos os campos');
      return;
    }

    setLoading(true);
    showToast('info', 'Verificando permissões de acesso...');

    const hasAccess = await checkAccess({
      patientAddress: patientId,
      doctorAddress: doctorId,
    });

    if (!hasAccess) {
      showToast('error', 'Você não tem acesso a este histórico');
      setLoading(false);
      return;
    }

    try {
      const result = await readPatientData({
        fileHashHex: cid,
        doctorAddress: doctorId,
        patientAddress: patientId,
      }).catch(err => err);

      if (!result.success) {
        showToast(
          'error',
          'Acesso negado pela HealthChain - Não tem permissão para visualizar os registros desse paciente',
        );
        setLoading(false);
        return;
      }

      showToast('success', 'Acesso concedido! Abrindo registro do paciente...');

      await openFile(cid);
    } catch (err) {
      console.log('Error checking access:', err);
      showToast(
        'error',
        'Verifica o de permissão falhou. Por favor, tente novamente.',
      );
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className='max-w-2xl mx-auto'>
      <div className='bg-white/70 backdrop-blur-md rounded-2xl shadow-xl p-8 border border-teal-100'>
        <Header />

        <div className='bg-blue-50 border border-blue-200 rounded-xl p-4 mb-6 flex items-start gap-3'>
          <Shield className='w-5 h-5 text-blue-600 mt-0.5 flex-shrink-0' />
          <p className='text-sm text-blue-800'>
            Esta tela verifica permissões on-chain antes de permitir acesso a
            registros de pacientes. Somente médicos autorizados podem visualizar
            dados de pacientes.
          </p>
        </div>

        <form onSubmit={handleVerifyAndView} className='space-y-6'>
          <Input
            value={doctorId}
            onChange={setDoctorId}
            title='Conta do Médico (Visualizador)'
          />

          <Input
            value={patientId}
            onChange={setPatientId}
            title='Conta do Paciente (Dono)'
          />

          <Input
            value={cid}
            onChange={setCid}
            placeholder='Qm...'
            title='Hash do Arquivo / CID (Arquivo Alvo)'
          />

          <button
            type='submit'
            disabled={loading}
            className='w-full bg-gradient-to-r from-teal-600 to-blue-500 text-white py-4 rounded-xl font-semibold hover:shadow-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2'
          >
            {loading ? (
              <>
                <Loader2 className='w-5 h-5 animate-spin' />
                Verificando Acesso...
              </>
            ) : (
              <>
                <ExternalLink className='w-5 h-5' />
                Verificar Acesso & Visualizar
              </>
            )}
          </button>
        </form>
      </div>
    </div>
  );
}
