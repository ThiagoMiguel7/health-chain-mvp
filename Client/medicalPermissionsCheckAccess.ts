// check-access-medical.ts
import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';

const WS = process.env.WS || 'ws://127.0.0.1:9944';

// Defaults (substitua se quiser)
const DEFAULT_DOCTOR_ADDRESS =
  '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty'; // exemplo (Bob)
// Você pode definir:
// - PATIENT_URI (ex: //Alice) para mudar o patient origem, ou usa //Alice por padrão
// - DOCTOR (endereço SS58) ou DOCTOR_URI (seed like //Bob)
const PATIENT_URI = process.env.PATIENT_URI || '//Alice';
const DOCTOR_ENV = process.env.DOCTOR; // ex: 5F...
const DOCTOR_URI = process.env.DOCTOR_URI; // ex: //Bob

async function main(): Promise<void> {
  const provider = new WsProvider(WS);
  const api = await ApiPromise.create({ provider });
  console.log('Conectado ao nó em', WS);

  const keyring = new Keyring({ type: 'sr25519' });

  // Conta paciente (Alice) — queremos o endereço público para consultar a storage
  const patientPair = keyring.addFromUri(PATIENT_URI);
  const patientAddr = patientPair.address;
  console.log('Paciente (patient) address:', patientAddr);

  // Obter endereço do doctor (Bob): prefer DOCTOR_URI -> DOCTOR -> DEFAULT
  let doctorAddr: string;
  if (DOCTOR_URI) {
    const doctorPair = keyring.addFromUri(DOCTOR_URI);
    doctorAddr = doctorPair.address;
  } else if (DOCTOR_ENV) {
    doctorAddr = DOCTOR_ENV;
  } else {
    doctorAddr = DEFAULT_DOCTOR_ADDRESS;
  }
  console.log('Doctor address:', doctorAddr);

  // Verifica se a storage query existe no runtime
  const queryAny = api.query as unknown as Record<string, any>;
  if (!queryAny.medicalPermissions || !queryAny.medicalPermissions.permissions) {
    console.error('api.query.medicalPermissions.permissions não encontrada no runtime.');
    console.error('Verifique o nome do pallet/storage no metadata do seu nó.');
    await api.disconnect();
    process.exit(1);
  }

  // Reproduz a mesma lógica do trait: se patient === doctor -> true
  if (patientAddr === doctorAddr) {
    console.log(`Resultado: true (patient === doctor — acesso sempre garantido pela política do pallet)`);
    await api.disconnect();
    process.exit(0);
  }

  try {
    // Ler diretamente a storage double map: permissions(patient, doctor)
    const stored = await api.query.medicalPermissions.permissions(patientAddr, doctorAddr);
    // toJSON() deve retornar boolean verdadeiro/falso para o tipo bool do storage
    const hasAccess = (stored.toJSON() as boolean) === true;

    console.log(
      `Has access from patient ${patientAddr} to doctor ${doctorAddr}? -> ${hasAccess}`
    );
  } catch (err) {
    console.error('Erro ao consultar a storage:', err);
  } finally {
    await api.disconnect();
  }
}

main().catch((err) => {
  console.error('Erro geral:', err);
  process.exit(1);
});

