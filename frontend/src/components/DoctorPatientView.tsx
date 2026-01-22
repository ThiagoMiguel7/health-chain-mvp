import { useState } from 'react';
import { Stethoscope, Loader2, ExternalLink, Shield } from 'lucide-react';
import { useToast } from '../contexts/ToastContext';
import { mockCheckPermission } from '../utils/blockchain';

export const DoctorPatientView = () => {
  const [doctorId, setDoctorId] = useState('');
  const [patientId, setPatientId] = useState('');
  const [cid, setCid] = useState('');
  const [loading, setLoading] = useState(false);
  const { showToast } = useToast();

  const handleVerifyAndView = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!doctorId || !patientId || !cid) {
      showToast('error', 'Please fill in all fields');
      return;
    }

    setLoading(true);
    showToast('info', 'Checking access permissions...');

    try {
      const hasPermission = await mockCheckPermission(doctorId, patientId);

      if (hasPermission) {
        const ipfsUrl = `https://ipfs.io/ipfs/${cid}`;
        showToast('success', 'Access granted! Opening patient record...');
        window.open(ipfsUrl, '_blank');
      } else {
        showToast('error', 'Access Denied by HealthChain - No permission to view this patient\'s records');
      }
    } catch (error) {
      showToast('error', 'Permission check failed. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="max-w-2xl mx-auto">
      <div className="bg-white/70 backdrop-blur-md rounded-2xl shadow-xl p-8 border border-teal-100">
        <div className="flex items-center gap-3 mb-6">
          <div className="p-3 bg-gradient-to-br from-teal-600 to-blue-500 rounded-xl">
            <Stethoscope className="w-6 h-6 text-white" />
          </div>
          <h2 className="text-2xl font-bold text-gray-800">Busca de Histórico de Paciente</h2>
        </div>

        <div className="bg-blue-50 border border-blue-200 rounded-xl p-4 mb-6 flex items-start gap-3">
          <Shield className="w-5 h-5 text-blue-600 mt-0.5 flex-shrink-0" />
          <p className="text-sm text-blue-800">
            This screen verifies on-chain permissions before allowing access to patient records.
            Only authorized doctors can view patient data.
          </p>
        </div>

        <form onSubmit={handleVerifyAndView} className="space-y-6">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Doctor Account ID (Viewer)
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
              Patient Account ID (Owner)
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
              File Hash / CID (Target File)
            </label>
            <input
              type="text"
              value={cid}
              onChange={(e) => setCid(e.target.value)}
              placeholder="Qm..."
              className="w-full px-4 py-3 bg-white border border-gray-300 rounded-xl focus:ring-2 focus:ring-teal-500 focus:border-transparent transition-all"
            />
          </div>

          <button
            type="submit"
            disabled={loading}
            className="w-full bg-gradient-to-r from-teal-600 to-blue-500 text-white py-4 rounded-xl font-semibold hover:shadow-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
          >
            {loading ? (
              <>
                <Loader2 className="w-5 h-5 animate-spin" />
                Verifying Access...
              </>
            ) : (
              <>
                <ExternalLink className="w-5 h-5" />
                Verify Access & View
              </>
            )}
          </button>
        </form>
      </div>
    </div>
  );
};
