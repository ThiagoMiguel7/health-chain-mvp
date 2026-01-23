// frstConnectionTest.ts
import { ApiPromise, WsProvider } from '@polkadot/api';

async function main(): Promise<void> {
  // Initialise the provider to connect to the local node
  const provider = new WsProvider('ws://127.0.0.1:9944');

  // Create the API and wait until ready
  const api = await ApiPromise.create({ provider });

  // Retrieve the chain & node information via rpc calls
  const [chain, nodeName, nodeVersion] = await Promise.all([
    api.rpc.system.chain(),
    api.rpc.system.name(),
    api.rpc.system.version(),
  ]);

  console.log(
    `You are connected to chain ${chain.toString()} using ${nodeName.toString()} v${nodeVersion.toString()}`
  );

  // Disconnect cleanly
  await api.disconnect();
}

main().catch((err) => {
  console.error('Erro:', err);
  process.exit(1);
});

