// read-own-data.ts
import { ApiPromise, WsProvider, SubmittableResult } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';

const WS = process.env.WS || 'ws://127.0.0.1:9944';

// Seeds / URIs das contas dev (padrões)
const ALICE_URI = process.env.PATIENT_URI || '//Alice';

// File hash (mesmo que você usou antes)
const FILE_HASH_HEX = '0x4933794f6a39737159443031596959';

async function main(): Promise<void> {
  const provider = new WsProvider(WS);
  const api = await ApiPromise.create({ provider });
  console.log('Conectado ao nó em', WS);

  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri(ALICE_URI);

  console.log('Originador (Alice) address:', alice.address);
  console.log('File hash (hex):', FILE_HASH_HEX);

  // Defensive access to api.tx
  const txAny = api.tx as unknown as Record<string, any>;
  const pallet = txAny.medicalHistoryReader;
  if (!pallet) {
    console.error('api.tx.medicalHistoryReader não encontrada no runtime.');
    await api.disconnect();
    process.exit(1);
  }

  // detecta tanto camelCase quanto snake_case
  const methodName =
    typeof pallet.readOwnData === 'function'
      ? 'readOwnData'
      : typeof pallet.read_own_data === 'function'
      ? 'read_own_data'
      : null;

  if (!methodName) {
    console.error(
      'readOwnData / read_own_data não encontrada em api.tx.medicalHistoryReader. Verifique o metadata do nó.'
    );
    await api.disconnect();
    process.exit(1);
  }

  console.log(`Chamando api.tx.medicalHistoryReader.${methodName}(${FILE_HASH_HEX})`);

  const extrinsic = pallet[methodName](FILE_HASH_HEX);

  try {
    const unsub = await extrinsic.signAndSend(alice, (result: SubmittableResult) => {
      const { status, events, dispatchError } = result;
      console.log('Status:', status.type);

      if (status.isInBlock) {
        console.log('Incluído no bloco:', status.asInBlock.toHex());
      }

      // dispatch error handling
      if (dispatchError) {
        if ((dispatchError as any).isModule) {
          try {
            const decoded = api.registry.findMetaError((dispatchError as any).asModule);
            const { section, name, docs } = decoded;
            console.error(`Erro do runtime: ${section}.${name} — ${docs.join(' ')}`);
          } catch (e) {
            console.error('Erro ao decodificar dispatchError do módulo:', e);
          }
        } else {
          console.error('Erro:', dispatchError.toString());
        }
      }

      // Varre eventos e procura OwnDataAccessed
      if (events && events.length) {
        events.forEach(({ event: { section, method, data }, phase }: any) => {
          console.log(`Event: ${section}.${method} (phase=${phase.toString()}) ->`, data.toString());

          if (section === 'medicalHistoryReader' && method === 'OwnDataAccessed') {
            try {
              // No Rust: OwnDataAccessed { patient: AccountId, file_hash: FileHash }
              const patientFromEvent = data[0].toString();
              const fileHashFromEvent = data[1].toString();

              console.log('--- OwnDataAccessed event captured ---');
              console.log('patient:', patientFromEvent);
              console.log('file_hash:', fileHashFromEvent);
              console.log('-------------------------------------');
            } catch (e) {
              console.error('Falha ao interpretar OwnDataAccessed event data:', e);
            }
          }

          // fallback: se quiser capturar só pelo nome do método do evento
          if (method === 'OwnDataAccessed' && section !== 'medicalHistoryReader') {
            console.log('Evento OwnDataAccessed detectado em outro section:', section);
          }
        });
      }

      if (status.isFinalized) {
        console.log('Finalizado no bloco:', status.asFinalized.toHex());
        // unsubscribe
        try {
          if (typeof unsub === 'function') {
            unsub();
          } else if (unsub && typeof (unsub as any).unsubscribe === 'function') {
            (unsub as any).unsubscribe();
          }
        } catch {
          // ignore
        }
        api.disconnect().catch(() => {});
        process.exit(0);
      }
    });

    // Alguns ambientes retornam hash diretamente
    if (unsub && typeof (unsub as any).toHex === 'function') {
      console.log('Hash da transação:', (unsub as any).toHex());
    }
  } catch (err) {
    console.error('Erro ao enviar extrinsic:', err);
    await api.disconnect();
    process.exit(1);
  }
}

main().catch((err) => {
  console.error('Erro geral:', err);
  process.exit(1);
});

