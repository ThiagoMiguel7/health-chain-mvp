import { useState, JSX, ChangeEvent, FormEvent } from 'react';
import { Upload, Loader2, CheckCircle, Copy, FileText } from 'lucide-react';

import { useToast } from '../contexts/ToastContext';
import { copyToClipboard } from '../utils/blockchain';
import { Input } from './Input';
import { uploadFileToIpfs } from '../utils/ipfs-functions';
import { checkAccess, createRecord } from '../utils/polkadot.api';
import { delay } from '../utils/promises';

interface UploadResult {
  cid: string;
  txHash: string;
  blockNumber: number;
}

type UploadState = 'idle' | 'ipfs' | 'blockchain' | 'complete';

function Header(): JSX.Element {
  return (
    <div className='flex items-center gap-3 mb-6'>
      <div className='p-3 bg-gradient-to-br from-blue-600 to-teal-500 rounded-xl'>
        <Upload className='w-6 h-6 text-white' />
      </div>

      <h2 className='text-2xl font-bold text-gray-800'>
        Criar Novo Registro Médico
      </h2>
    </div>
  );
}

export function CreateRecord(): JSX.Element {
  const { showToast } = useToast();

  const [doctorId, setDoctorId] = useState('');
  const [loading, setLoading] = useState(false);
  const [patientId, setPatientId] = useState('');
  const [file, setFile] = useState<File | null>(null);
  const [result, setResult] = useState<UploadResult | null>(null);
  const [uploadStep, setUploadStep] = useState<UploadState>('idle');

  const handleFileChange = (e: ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files[0]) {
      setFile(e.target.files[0]);
      setResult(null);
      setUploadStep('idle');
    }
  };

  const handleCopy = async (text: string, label: string) => {
    await copyToClipboard(text);
    showToast('success', `Copiado para area de transferência! ${label}`);
  };

  const handleUpload = async (e: FormEvent) => {
    e.preventDefault();

    if (!file || !patientId || !doctorId) {
      showToast(
        'error',
        'Por favor, preencha todos os campos e selecione um arquivo',
      );
      return;
    }

    setLoading(true);

    const hasAccess = await checkAccess({
      patientAddress: patientId,
      doctorAddress: doctorId,
    });

    if (!hasAccess) {
      showToast('error', 'Você não tem acesso a este paciente');
      setLoading(false);
      return;
    }

    try {
      setUploadStep('ipfs');
      showToast('info', 'Etapa 1: Enviando arquivo para o IPFS...');

      const cid = await uploadFileToIpfs(file);

      showToast(
        'success',
        `Arquivo enviado para o IPFS! CID: ${cid.slice(0, 15)}...`,
      );

      await delay();

      setUploadStep('blockchain');
      showToast('info', 'Etapa 2: Gravando na blockchain...');

      const result = await createRecord({
        fileHashHex: cid,
        doctorAddress: doctorId,
        patientAddress: patientId,
      });

      if (!result.success) {
        throw result.error;
      }

      const txHash = result.transactionHash;
      const blockNumber = result.blockNumber;

      setUploadStep('complete');
      setResult({ cid, txHash, blockNumber });

      showToast('success', 'Registro médico criado com sucesso!');
    } catch (err) {
      showToast('error', 'Falha no upload. Por favor, tente novamente.');
      setUploadStep('idle');
      console.log('Error uploading file:', err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className='max-w-4xl mx-auto'>
      <div className='bg-white/70 backdrop-blur-md rounded-2xl shadow-xl p-8 border border-blue-100'>
        <Header />

        <form onSubmit={handleUpload} className='space-y-6'>
          <div className='grid md:grid-cols-2 gap-6'>
            <div className='md:col-span-2'>
              <label className='block text-sm font-medium text-gray-700 mb-2'>
                Arquivo Médico
              </label>
              <div className='relative'>
                <input
                  type='file'
                  id='file-upload'
                  className='hidden'
                  onChange={handleFileChange}
                />
                <label
                  htmlFor='file-upload'
                  className='flex items-center justify-center w-full px-6 py-12 border-2 border-dashed border-gray-300 rounded-xl cursor-pointer hover:border-teal-400 transition-colors bg-gradient-to-br from-gray-50 to-blue-50'
                >
                  <div className='text-center'>
                    <FileText className='w-12 h-12 mx-auto text-gray-400 mb-3' />
                    <p className='text-sm text-gray-600'>
                      {file ? (
                        <span className='font-medium text-teal-600'>
                          {file.name}
                        </span>
                      ) : (
                        <>
                          Clique para selecionar um arquivo ou arraste e solte
                        </>
                      )}
                    </p>
                    <p className='text-xs text-gray-500 mt-1'>
                      PDF, DICOM ou qualquer arquivo médico
                    </p>
                  </div>
                </label>
              </div>
            </div>

            <Input
              value={patientId}
              onChange={setPatientId}
              title='ID da Conta do Paciente'
            />

            <Input
              value={doctorId}
              onChange={setDoctorId}
              title='ID da Conta do Médico'
            />
          </div>

          {loading && (
            <div className='bg-blue-50 border border-blue-200 rounded-xl p-4'>
              <div className='flex items-center gap-3 mb-2'>
                <Loader2 className='w-5 h-5 text-blue-600 animate-spin' />
                <span className='text-sm font-medium text-blue-800'>
                  {uploadStep === 'ipfs' && 'Enviando para IPFS...'}
                  {uploadStep === 'blockchain' && 'Gravando no Blockchain...'}
                </span>
              </div>
              {uploadStep === 'ipfs' && (
                <div className='w-full h-2 bg-blue-200 rounded-full overflow-hidden'>
                  <div
                    className='h-full bg-blue-500 animate-infinite-progress'
                    style={{ width: '100%' }}
                  />
                </div>
              )}
            </div>
          )}

          <button
            type='submit'
            disabled={loading || uploadStep === 'complete'}
            className='w-full bg-gradient-to-r from-blue-600 to-teal-500 text-white py-4 rounded-xl font-semibold hover:shadow-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2'
          >
            {loading ? (
              <>
                <Loader2 className='w-5 h-5 animate-spin' />
                Processando...
              </>
            ) : uploadStep === 'complete' ? (
              <>
                <CheckCircle className='w-5 h-5' />
                Upload Concluído
              </>
            ) : (
              <>
                <Upload className='w-5 h-5' />
                Enviar & Registrar
              </>
            )}
          </button>
        </form>

        {result && (
          <div className='mt-8 bg-gradient-to-br from-green-50 to-teal-50 border border-green-200 rounded-xl p-6'>
            <div className='flex items-center gap-2 mb-4'>
              <CheckCircle className='w-6 h-6 text-green-600' />
              <h3 className='text-lg font-bold text-gray-800'>
                Sucesso! Registro criado com sucesso
              </h3>
            </div>

            <div className='space-y-3'>
              <div className='bg-white rounded-lg p-4'>
                <label className='text-xs font-medium text-gray-500 uppercase mb-1 block'>
                  CID IPFS
                </label>
                <div className='flex items-center gap-2'>
                  <code className='flex-1 text-sm text-gray-800 font-mono break-all'>
                    {result.cid}
                  </code>
                  <button
                    onClick={() => handleCopy(result.cid, 'CID')}
                    className='p-2 hover:bg-gray-100 rounded-lg transition-colors'
                  >
                    <Copy className='w-4 h-4 text-gray-600' />
                  </button>
                </div>
              </div>

              <div className='bg-white rounded-lg p-4'>
                <label className='text-xs font-medium text-gray-500 uppercase mb-1 block'>
                  Hash da Transação
                </label>
                <div className='flex items-center gap-2'>
                  <code className='flex-1 text-sm text-gray-800 font-mono break-all'>
                    {result.txHash}
                  </code>
                  <button
                    onClick={() =>
                      handleCopy(result.txHash, 'Hash da Transação')
                    }
                    className='p-2 hover:bg-gray-100 rounded-lg transition-colors'
                  >
                    <Copy className='w-4 h-4 text-gray-600' />
                  </button>
                </div>
              </div>

              <div className='bg-white rounded-lg p-4'>
                <label className='text-xs font-medium text-gray-500 uppercase mb-1 block'>
                  Número do Bloco
                </label>
                <code className='text-sm text-gray-800 font-mono'>
                  #{result.blockNumber.toLocaleString()}
                </code>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
