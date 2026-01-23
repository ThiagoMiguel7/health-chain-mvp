// grant-access-medical.js
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');

const WS = process.env.WS || 'ws://127.0.0.1:9944';

// Endereço do doctor (alvo da permissão). Substitua se necessário.
const DOCTOR = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty'; // exemplo (Bob)

async function main() {
  const provider = new WsProvider(WS);
  const api = await ApiPromise.create({ provider });
  console.log('Conectado ao nó em', WS);

  // Keyring e conta originadora (paciente = Alice)
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');

  // Verifica se a call existe
  if (!api.tx.medicalPermissions || !api.tx.medicalPermissions.grantAccess) {
    console.error('api.tx.medicalPermissions.grantAccess não encontrada no runtime.');
    console.error('Confirme o nome do pallet/extrinsic no metadata do seu nó.');
    await api.disconnect();
    process.exit(1);
  }

  console.log('Chamando api.tx.medicalPermissions.grantAccess(', DOCTOR, ')');

  const extrinsic = api.tx.medicalPermissions.grantAccess(DOCTOR);

  try {
    const unsub = await extrinsic.signAndSend(alice, ({ status, events, dispatchError }) => {
      console.log('Status:', status.type);

      if (status.isInBlock) {
        console.log('Incluído no bloco:', status.asInBlock.toHex());
      }

      if (dispatchError) {
        if (dispatchError.isModule) {
          // Decodifica erro do runtime
          const decoded = api.registry.findMetaError(dispatchError.asModule);
          const { section, name, docs } = decoded;
          console.error(`Erro do runtime: ${section}.${name} — ${docs.join(' ')}`);
        } else {
          console.error('Erro:', dispatchError.toString());
        }
      }

      if (events && events.length) {
        events.forEach(({ event: { section, method, data }, phase }) => {
          console.log(`Event: ${section}.${method} (phase=${phase.toString()}) ->`, data.toString());
        });
      }

      if (status.isFinalized) {
        console.log('Finalizado no bloco:', status.asFinalized.toHex());
        unsub();
        api.disconnect().catch(() => {});
        process.exit(0);
      }
    });

    // Alguns ambientes retornam hash direto; logue se for o caso
    if (unsub && typeof unsub === 'object' && unsub.toHex) {
      console.log('Hash da transação:', unsub.toHex());
    }
  } catch (err) {
    console.error('Erro ao enviar extrinsic:', err);
    await api.disconnect();
    process.exit(1);
  }
}

main().catch(err => {
  console.error('Erro geral:', err);
  process.exit(1);
});

