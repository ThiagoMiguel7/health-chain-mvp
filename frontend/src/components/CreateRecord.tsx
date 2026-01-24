import { useState } from 'react';
import { Upload, Loader2, CheckCircle, Copy, FileText } from 'lucide-react';
import { useToast } from '../contexts/ToastContext';
import { mockIPFSUpload, mockCreateRecord, copyToClipboard } from '../utils/blockchain';

interface UploadResult {
  cid: string;
  txHash: string;
  blockNumber: number;
}

export const CreateRecord = () => {
  const [file, setFile] = useState<File | null>(null);
  const [patientId, setPatientId] = useState('');
  const [doctorId, setDoctorId] = useState('');
  const [loading, setLoading] = useState(false);
  const [uploadStep, setUploadStep] = useState<'idle' | 'ipfs' | 'blockchain' | 'complete'>('idle');
  const [result, setResult] = useState<UploadResult | null>(null);
  const [progress, setProgress] = useState(0);
  const { showToast } = useToast();

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files[0]) {
      setFile(e.target.files[0]);
      setResult(null);
      setUploadStep('idle');
    }
  };

  const handleCopy = async (text: string, label: string) => {
    await copyToClipboard(text);
    showToast('success', `${label} copied to clipboard!`);
  };

  const handleUpload = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!file || !patientId || !doctorId) {
      showToast('error', 'Please fill in all fields and select a file');
      return;
    }

    setLoading(true);
    setProgress(0);

    try {
      setUploadStep('ipfs');
      showToast('info', 'Step 1: Uploading file to IPFS...');

      const progressInterval = setInterval(() => {
        setProgress(prev => Math.min(prev + 10, 90));
      }, 250);

      const cid = await mockIPFSUpload(file);
      clearInterval(progressInterval);
      setProgress(100);

      showToast('success', `File uploaded to IPFS! CID: ${cid.slice(0, 15)}...`);

      await new Promise(resolve => setTimeout(resolve, 500));

      setUploadStep('blockchain');
      setProgress(0);
      showToast('info', 'Step 2: Recording on blockchain...');

      const { txHash, blockNumber } = await mockCreateRecord(patientId, doctorId, cid);

      setUploadStep('complete');
      setResult({ cid, txHash, blockNumber });

      showToast('success', 'Medical record created successfully!');
    } catch (error) {
      showToast('error', 'Upload failed. Please try again.');
      setUploadStep('idle');
    } finally {
      setLoading(false);
      setProgress(0);
    }
  };

  return (
    <div className="max-w-4xl mx-auto">
      <div className="bg-white/70 backdrop-blur-md rounded-2xl shadow-xl p-8 border border-blue-100">
        <div className="flex items-center gap-3 mb-6">
          <div className="p-3 bg-gradient-to-br from-blue-600 to-teal-500 rounded-xl">
            <Upload className="w-6 h-6 text-white" />
          </div>
          <h2 className="text-2xl font-bold text-gray-800">Criar Novo Registro Médico</h2>
        </div>

        <form onSubmit={handleUpload} className="space-y-6">
          <div className="grid md:grid-cols-2 gap-6">
            <div className="md:col-span-2">
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Medical File
              </label>
              <div className="relative">
                <input
                  type="file"
                  id="file-upload"
                  onChange={handleFileChange}
                  className="hidden"
                />
                <label
                  htmlFor="file-upload"
                  className="flex items-center justify-center w-full px-6 py-12 border-2 border-dashed border-gray-300 rounded-xl cursor-pointer hover:border-teal-400 transition-colors bg-gradient-to-br from-gray-50 to-blue-50"
                >
                  <div className="text-center">
                    <FileText className="w-12 h-12 mx-auto text-gray-400 mb-3" />
                    <p className="text-sm text-gray-600">
                      {file ? (
                        <span className="font-medium text-teal-600">{file.name}</span>
                      ) : (
                        <>Click to select a file or drag and drop</>
                      )}
                    </p>
                    <p className="text-xs text-gray-500 mt-1">PDF, DICOM, or any medical file</p>
                  </div>
                </label>
              </div>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Patient Account ID
              </label>
              <input
                type="text"
                value={patientId}
                onChange={(e) => setPatientId(e.target.value)}
                placeholder="5Grw..."
                className="w-full px-4 py-3 bg-white border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Doctor Account ID
              </label>
              <input
                type="text"
                value={doctorId}
                onChange={(e) => setDoctorId(e.target.value)}
                placeholder="5Grw..."
                className="w-full px-4 py-3 bg-white border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
              />
            </div>
          </div>

          {loading && (
            <div className="bg-blue-50 border border-blue-200 rounded-xl p-4">
              <div className="flex items-center gap-3 mb-2">
                <Loader2 className="w-5 h-5 text-blue-600 animate-spin" />
                <span className="text-sm font-medium text-blue-800">
                  {uploadStep === 'ipfs' && 'Uploading to IPFS...'}
                  {uploadStep === 'blockchain' && 'Recording on Blockchain...'}
                </span>
              </div>
              {uploadStep === 'ipfs' && (
                <div className="w-full bg-blue-200 rounded-full h-2">
                  <div
                    className="bg-blue-600 h-2 rounded-full transition-all duration-300"
                    style={{ width: `${progress}%` }}
                  />
                </div>
              )}
            </div>
          )}

          <button
            type="submit"
            disabled={loading || uploadStep === 'complete'}
            className="w-full bg-gradient-to-r from-blue-600 to-teal-500 text-white py-4 rounded-xl font-semibold hover:shadow-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
          >
            {loading ? (
              <>
                <Loader2 className="w-5 h-5 animate-spin" />
                Processing...
              </>
            ) : uploadStep === 'complete' ? (
              <>
                <CheckCircle className="w-5 h-5" />
                Upload Complete
              </>
            ) : (
              <>
                <Upload className="w-5 h-5" />
                Upload & Register
              </>
            )}
          </button>
        </form>

        {result && (
          <div className="mt-8 bg-gradient-to-br from-green-50 to-teal-50 border border-green-200 rounded-xl p-6">
            <div className="flex items-center gap-2 mb-4">
              <CheckCircle className="w-6 h-6 text-green-600" />
              <h3 className="text-lg font-bold text-gray-800">Success! Record Created</h3>
            </div>

            <div className="space-y-3">
              <div className="bg-white rounded-lg p-4">
                <label className="text-xs font-medium text-gray-500 uppercase mb-1 block">
                  IPFS CID
                </label>
                <div className="flex items-center gap-2">
                  <code className="flex-1 text-sm text-gray-800 font-mono break-all">
                    {result.cid}
                  </code>
                  <button
                    onClick={() => handleCopy(result.cid, 'CID')}
                    className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
                  >
                    <Copy className="w-4 h-4 text-gray-600" />
                  </button>
                </div>
              </div>

              <div className="bg-white rounded-lg p-4">
                <label className="text-xs font-medium text-gray-500 uppercase mb-1 block">
                  Transaction Hash
                </label>
                <div className="flex items-center gap-2">
                  <code className="flex-1 text-sm text-gray-800 font-mono break-all">
                    {result.txHash}
                  </code>
                  <button
                    onClick={() => handleCopy(result.txHash, 'Transaction Hash')}
                    className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
                  >
                    <Copy className="w-4 h-4 text-gray-600" />
                  </button>
                </div>
              </div>

              <div className="bg-white rounded-lg p-4">
                <label className="text-xs font-medium text-gray-500 uppercase mb-1 block">
                  Block Number
                </label>
                <code className="text-sm text-gray-800 font-mono">
                  #{result.blockNumber.toLocaleString()}
                </code>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
