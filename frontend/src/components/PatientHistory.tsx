import { useState } from 'react';
import { FileText, Loader2, ExternalLink } from 'lucide-react';
import { useToast } from '../contexts/ToastContext';
import { useWallet } from '../contexts/WalletContext';
import { mockVerifyRecord } from '../utils/blockchain';

export const PatientHistory = () => {
  const [accountId, setAccountId] = useState('');
  const [cid, setCid] = useState('');
  const [loading, setLoading] = useState(false);
  const { showToast } = useToast();
  const { accountId: walletAccountId } = useWallet();

  const handleView = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!accountId || !cid) {
      showToast('error', 'Please fill in all fields');
      return;
    }

    setLoading(true);
    showToast('info', 'Verifying record on-chain...');

    try {
      const exists = await mockVerifyRecord(cid);

      if (exists) {
        const ipfsUrl = `https://ipfs.io/ipfs/${cid}`;
        showToast('success', 'Record verified! Opening IPFS gateway...');
        window.open(ipfsUrl, '_blank');
      } else {
        showToast('error', 'Record not found on blockchain');
      }
    } catch (error) {
      showToast('error', 'Verification failed. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="max-w-2xl mx-auto">
      <div className="bg-white/70 backdrop-blur-md rounded-2xl shadow-xl p-8 border border-blue-100">
        <div className="flex items-center gap-3 mb-6">
          <div className="p-3 bg-gradient-to-br from-blue-500 to-teal-600 rounded-xl">
            <FileText className="w-6 h-6 text-white" />
          </div>
          <h2 className="text-2xl font-bold text-gray-800">Recuperação de Histórico Próprio</h2>
        </div>

        <form onSubmit={handleView} className="space-y-6">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              My Account ID
            </label>
            <input
              type="text"
              value={accountId}
              onChange={(e) => setAccountId(e.target.value)}
              placeholder={walletAccountId || "5Grw..."}
              className="w-full px-4 py-3 bg-white border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
            />
            {walletAccountId && (
              <button
                type="button"
                onClick={() => setAccountId(walletAccountId)}
                className="mt-2 text-sm text-blue-600 hover:text-blue-700 font-medium"
              >
                Use connected wallet: {walletAccountId.slice(0, 8)}...
              </button>
            )}
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              File Hash / CID
            </label>
            <input
              type="text"
              value={cid}
              onChange={(e) => setCid(e.target.value)}
              placeholder="Qm..."
              className="w-full px-4 py-3 bg-white border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
            />
          </div>

          <button
            type="submit"
            disabled={loading}
            className="w-full bg-gradient-to-r from-blue-500 to-teal-600 text-white py-4 rounded-xl font-semibold hover:shadow-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
          >
            {loading ? (
              <>
                <Loader2 className="w-5 h-5 animate-spin" />
                Verifying...
              </>
            ) : (
              <>
                <ExternalLink className="w-5 h-5" />
                Search & View
              </>
            )}
          </button>
        </form>
      </div>
    </div>
  );
};
