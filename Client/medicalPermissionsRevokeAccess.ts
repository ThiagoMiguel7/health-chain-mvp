// revoke-access-medical.ts
import { ApiPromise, WsProvider, SubmittableResult } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';

const WS = process.env.WS || 'ws://127.0.0.1:9944';

// Endereço do doctor cujo acesso será revogado (Bob)
const DOCTOR = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';

async function main(): Promise<void> {
  const provider = new WsProvider(WS);
  const api = await ApiPromise.create({ provider });
  console.log('Conectado ao nó em', WS);

  // Conta originadora (paciente = Alice)
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');

  // Evita problemas de tipagem com pallets customizados
  const txAny = api.tx as unknown as Record<string, any>;
  if (!txAny.medicalPermissions || !txAny.medicalPermissions.revokeAccess) {
    console.error('api.tx.medicalPermissions.revokeAccess não encontrada no runtime.');
    console.error('Confirme o nome do pallet/extrinsic no metadata do seu nó.');
    await api.disconnect();
    process.exit(1);
  }

  console.log('Chamando api.tx.medicalPermissions.revokeAccess(', DOCTOR, ')');

  const extrinsic = txAny.medicalPermissions.revokeAccess(DOCTOR);

  try {
    const unsub = await extrinsic.signAndSend(alice, (result: SubmittableResult) => {
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

      // Eventos emitidos
      if (events && events.length) {
        events.forEach(({ event: { section, method, data }, phase }: any) => {
          console.log(
            `Event: ${section}.${method} (phase=${phase.toString()}) ->`,
            data.toString()
          );
        });
      }

      if (status.isFinalized) {
        console.log('Finalizado no bloco:', status.asFinalized.toHex());
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

    // Log defensivo caso retorne hash
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

