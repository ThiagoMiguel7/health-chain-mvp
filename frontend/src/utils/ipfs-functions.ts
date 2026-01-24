import { create } from 'ipfs-http-client';

const IPFS_URL = 'http://localhost:5001';

export const ipfs = create({ url: IPFS_URL });

export async function uploadFileToIpfs(file: File): Promise<string> {
  const { cid } = await ipfs.add(file);

  console.log('uploadFileToIpfs', { cid });

  return cid.toString();
}
