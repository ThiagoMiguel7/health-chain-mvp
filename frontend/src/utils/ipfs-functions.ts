import { create } from 'ipfs-http-client';

const IPFS_URL = 'http://127.0.0.1:5001';

export const ipfs = create({ url: IPFS_URL });
