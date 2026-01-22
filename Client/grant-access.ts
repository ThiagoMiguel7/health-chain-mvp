import { createClient } from 'polkadot-api';
import { getWsProvider } from 'polkadot-api/ws-provider/web';
import { healthchain } from '@polkadot-api/descriptors';
import { Keyring } from '@polkadot/keyring';

async function main() {
  // Conecta ao node
  const client = createClient(
    getWsProvider('ws://127.0.0.1:9944')
  );

  console.log('client keys:', Object.keys(client));
  console.log('client proto keys:', Object.getOwnPropertyNames(Object.getPrototypeOf(client)));


  // Typed API gerada pelo PAPI
  const typedApi = client.getTypedApi(healthchain);

  // Contas DEV (as MESMAS da Polkadot.js UI)
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');
  const bob = keyring.addFromUri('//Bob');

  // Extrinsic — nomes EXATOS do runtime
  const tx = typedApi.tx.MedicalPermissions.grant_access({
    doctor: bob.address
  });

  console.log('Enviando grant_access...');
  //await client.signAndSend(tx, alice, (result: any) => {
  const unsub = await tx.signAndSend(alice, ({ status, events }: any) => {
    console.log('Status:', status.toString());

    if (status.isInBlock) {
      console.log('📦 In block');
      events.forEach((e: any) => {
        console.log(e.toHuman?.() ?? e);
      });
    }

    if (status.isFinalized) {
      console.log('✅ Finalizado');
      unsub();
      process.exit(0);
    }
  });
}

main().catch(console.error);
