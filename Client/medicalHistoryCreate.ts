// create-record-medical.ts (corrigido)
import { ApiPromise, WsProvider, SubmittableResult } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';

/**
 * Configuração
 */
const WS = process.env.WS || 'ws://127.0.0.1:9944';

// Use URIs / seeds em vez de endereços hardcoded
const PATIENT_URI = process.env.PATIENT_URI || '//Alice'; // paciente (Alice)
const BOB_URI = process.env.BOB_URI || '//Bob'; // originador (Bob)

/**
 * Gera string aleatória alfanum de tamanho `len`
 */
function randomString(len: number): string {
  const chars = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';
  let s = '';
  for (let i = 0; i < len; i++) {
    s += chars.charAt(Math.floor(Math.random() * chars.length));
  }
  return s;
}

/**
 * Converte string ASCII para hex '0x...'
 */
function stringToHexAscii(s: string): string {
  return '0x' + Buffer.from(s, 'utf8').toString('hex');
}

async function main(): Promise<void> {
  const provider = new WsProvider(WS);
  const api = await ApiPromise.create({ provider });
  console.log('Conectado ao nó em', WS);

  // Keyring e pares
  const keyring = new Keyring({ type: 'sr25519' });
  const patientPair = keyring.addFromUri(PATIENT_URI);
  const bob = keyring.addFromUri(BOB_URI);

  const patientAddr = patientPair.address;
  const bobAddr = bob.address;

  console.log('Paciente (Alice) address (from URI):', patientAddr);
  console.log('Originador (Bob) address (from URI):', bobAddr);

  // Verifica existência da call
  const txAny = api.tx as unknown as Record<string, any>;
  if (!txAny.medicalHistory || !txAny.medicalHistory.createRecord) {
    console.error('api.tx.medicalHistory.createRecord não encontrada no runtime.');
    await api.disconnect();
    process.exit(1);
  }

  // Cria file_hash aleatório de 15 caracteres e converte para hex
  const random15 = randomString(15);
  const fileHashHex = stringToHexAscii(random15);
  console.log('Usando file_hash (15 chars):', random15);
  console.log('file_hash em hex:', fileHashHex);

  // Monta extrinsic: createRecord(patient, file_hash)
  const extrinsic = txAny.medicalHistory.createRecord(patientAddr, fileHashHex);

  try {
    const unsub = await extrinsic.signAndSend(bob, (result: SubmittableResult) => {
      const { status, events, dispatchError } = result;
      console.log('Status:', status.type);

      if (status.isInBlock) {
        console.log('Incluído no bloco:', status.asInBlock.toHex());
      }

      // Dispatch error handling
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

      // Eventos: filtra pelo pallet medicalHistory para buscar RecordCreated
      if (events && events.length) {
        events.forEach(({ event: { section, method, data }, phase }: any) => {
          console.log(`Event: ${section}.${method} (phase=${phase.toString()}) ->`, data.toString());

          if (section === 'medicalHistory' && method === 'RecordCreated') {
            console.log('RecordCreated detected ->', data.toString());
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

    // Defensive log se retornar hash/obj
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

