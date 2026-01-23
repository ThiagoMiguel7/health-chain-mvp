// read-patient-data.ts
import { ApiPromise, WsProvider, SubmittableResult } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';

const WS = process.env.WS || 'ws://127.0.0.1:9944';

// Seeds / URIs das contas dev (padrões)
// Use PATIENT_URI e BOB_URI via env se quiser substituir
const PATIENT_URI = process.env.PATIENT_URI || '//Alice';
const BOB_URI = process.env.BOB_URI || '//Bob';

// File hash fornecido (mesmo que você gerou antes)
const FILE_HASH_HEX = '0x4933794f6a39737159443031596959';

async function main(): Promise<void> {
  const provider = new WsProvider(WS);
  const api = await ApiPromise.create({ provider });
  console.log('Conectado ao nó em', WS);

  const keyring = new Keyring({ type: 'sr25519' });
  const patientPair = keyring.addFromUri(PATIENT_URI);
  const bob = keyring.addFromUri(BOB_URI);

  const patientAddr = patientPair.address;
  const bobAddr = bob.address;

  console.log('Paciente (Alice) address:', patientAddr);
  console.log('Originador (Bob) address:', bobAddr);
  console.log('File hash (hex):', FILE_HASH_HEX);

  // Evita problemas de tipagem com pallets customizados
  const txAny = api.tx as unknown as Record<string, any>;
  const pallet = txAny.medicalHistoryReader;
  if (!pallet) {
    console.error('api.tx.medicalHistoryReader não encontrada no runtime.');
    await api.disconnect();
    process.exit(1);
  }

  // Suporta tanto camelCase quanto snake_case na metadata
  const methodName =
    typeof pallet.readPatientData === 'function'
      ? 'readPatientData'
      : typeof pallet.read_patient_data === 'function'
      ? 'read_patient_data'
      : null;

  if (!methodName) {
    console.error(
      'readPatientData / read_patient_data não encontrada em api.tx.medicalHistoryReader. Verifique o metadata do nó.'
    );
    await api.disconnect();
    process.exit(1);
  }

  console.log(`Chamando api.tx.medicalHistoryReader.${methodName}(${patientAddr}, ${FILE_HASH_HEX})`);

  // Monta extrinsic: readPatientData(patient, file_hash)
  const extrinsic = pallet[methodName](patientAddr, FILE_HASH_HEX);

  try {
    const unsub = await extrinsic.signAndSend(bob, (result: SubmittableResult) => {
      const { status, events, dispatchError } = result;
      console.log('Status:', status.type);

      if (status.isInBlock) {
        console.log('Incluído no bloco:', status.asInBlock.toHex());
      }

      // Tratamento de erro de dispatch
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

      // Varre eventos e procura o PatientDataAccessed
      if (events && events.length) {
        events.forEach(({ event: { section, method, data }, phase }: any) => {
          console.log(`Event: ${section}.${method} (phase=${phase.toString()}) ->`, data.toString());

          // Evento esperado: PatientDataAccessed { doctor, patient, file_hash }
          // Observação: a ordem dos campos segue o emit no Rust (doctor, patient, file_hash)
          if (
            method === 'PatientDataAccessed'
          ) {
            try {
              const doctor = data[0].toString();
              const patientFromEvent = data[1].toString();
              const fileHashFromEvent = data[2].toString();

              console.log('--- PatientDataAccessed event captured ---');
              console.log('doctor:', doctor);
              console.log('patient:', patientFromEvent);
              console.log('file_hash:', fileHashFromEvent);
              console.log('----------------------------------------');
            } catch (e) {
              console.error('Falha ao interpretar PatientDataAccessed event data:', e);
            }
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

    // Caso algumas versões retornem hash direto
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

