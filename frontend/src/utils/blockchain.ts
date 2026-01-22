export const generateTxHash = (): string => {
  return '0x' + Array.from({ length: 64 }, () =>
    Math.floor(Math.random() * 16).toString(16)
  ).join('');
};

export const generateBlockNumber = (): number => {
  return Math.floor(Math.random() * 1000000) + 5000000;
};

export const generateCID = (): string => {
  const chars = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';
  let cid = 'Qm';
  for (let i = 0; i < 44; i++) {
    cid += chars.charAt(Math.floor(Math.random() * chars.length));
  }
  return cid;
};

export const mockPermissionTransaction = async (
  doctorId: string,
  patientId: string,
  action: 'grant' | 'revoke'
): Promise<{ txHash: string; blockNumber: number }> => {
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve({
        txHash: generateTxHash(),
        blockNumber: generateBlockNumber()
      });
    }, 2000);
  });
};

export const mockCheckPermission = async (
  doctorId: string,
  patientId: string
): Promise<boolean> => {
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve(Math.random() > 0.3);
    }, 1500);
  });
};

export const mockIPFSUpload = async (file: File): Promise<string> => {
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve(generateCID());
    }, 2500);
  });
};

export const mockCreateRecord = async (
  patientId: string,
  doctorId: string,
  cid: string
): Promise<{ txHash: string; blockNumber: number }> => {
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve({
        txHash: generateTxHash(),
        blockNumber: generateBlockNumber()
      });
    }, 1500);
  });
};

export const mockVerifyRecord = async (cid: string): Promise<boolean> => {
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve(true);
    }, 1000);
  });
};

export const copyToClipboard = async (text: string): Promise<void> => {
  try {
    await navigator.clipboard.writeText(text);
  } catch (err) {
    console.error('Failed to copy:', err);
  }
};
