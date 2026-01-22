import { useState } from 'react';
import { Lock, Loader2 } from 'lucide-react';
import { useToast } from '../contexts/ToastContext';
import { mockPermissionTransaction } from '../utils/blockchain';

export const PermissionManagement = () => {
  const [doctorId, setDoctorId] = useState('');
  const [patientId, setPatientId] = useState('');
  const [action, setAction] = useState<'grant' | 'revoke'>('grant');
  const [loading, setLoading] = useState(false);
  const { showToast } = useToast();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!doctorId || !patientId) {
      showToast('error', 'Please fill in all fields');
      return;
    }

    setLoading(true);
    showToast('info', 'Signing transaction...');

    try {
      const { txHash, blockNumber } = await mockPermissionTransaction(doctorId, patientId, action);
      showToast(
        'success',
        `Permission ${action === 'grant' ? 'granted' : 'revoked'} successfully! TxHash: ${txHash.slice(0, 10)}... | Block: ${blockNumber}`
      );
      setDoctorId('');
      setPatientId('');
    } catch (error) {
      showToast('error', 'Transaction failed. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="max-w-2xl mx-auto">
      <div className="bg-white/70 backdrop-blur-md rounded-2xl shadow-xl p-8 border border-teal-100">
        <div className="flex items-center gap-3 mb-6">
          <div className="p-3 bg-gradient-to-br from-teal-500 to-blue-600 rounded-xl">
            <Lock className="w-6 h-6 text-white" />
          </div>
          <h2 className="text-2xl font-bold text-gray-800">Conceder e Revogar Acesso</h2>
        </div>

        <form onSubmit={handleSubmit} className="space-y-6">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Doctor Account ID
            </label>
            <input
              type="text"
              value={doctorId}
              onChange={(e) => setDoctorId(e.target.value)}
              placeholder="5Grw..."
              className="w-full px-4 py-3 bg-white border border-gray-300 rounded-xl focus:ring-2 focus:ring-teal-500 focus:border-transparent transition-all"
            />
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
              className="w-full px-4 py-3 bg-white border border-gray-300 rounded-xl focus:ring-2 focus:ring-teal-500 focus:border-transparent transition-all"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Action
            </label>
            <select
              value={action}
              onChange={(e) => setAction(e.target.value as 'grant' | 'revoke')}
              className="w-full px-4 py-3 bg-white border border-gray-300 rounded-xl focus:ring-2 focus:ring-teal-500 focus:border-transparent transition-all cursor-pointer"
            >
              <option value="grant">Grant Access</option>
              <option value="revoke">Revoke Access</option>
            </select>
          </div>

          <button
            type="submit"
            disabled={loading}
            className="w-full bg-gradient-to-r from-teal-500 to-blue-600 text-white py-4 rounded-xl font-semibold hover:shadow-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
          >
            {loading ? (
              <>
                <Loader2 className="w-5 h-5 animate-spin" />
                Processing...
              </>
            ) : (
              'Submit Transaction'
            )}
          </button>
        </form>
      </div>
    </div>
  );
};
