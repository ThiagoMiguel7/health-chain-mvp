import { ApiPromise, WsProvider, SubmittableResult } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { web3Enable, web3FromSource } from '@polkadot/extension-dapp';

import { WS_URL } from './shared';
import { withResolvers } from './promises';

const provider = new WsProvider(WS_URL);
const api = await ApiPromise.create({ provider });

const keyring = new Keyring({ type: 'sr25519', ss58Format: 2 });

type CheckAccess = { patientUri: string; doctorUri: string };
type GrantAccess = { patientAddress: string; doctorAddress: string };

export async function checkAccess({
  patientUri,
  doctorUri,
}: Readonly<CheckAccess>): Promise<boolean> {
  const { address: patientAddress } = keyring.addFromUri(patientUri);
  const { address: doctorAddress } = keyring.addFromUri(doctorUri);

  console.log('🚀 ~ checkAccess ~ patientAddress:', patientAddress);
  console.log('🚀 ~ checkAccess ~ doctorAddress:', doctorAddress);

  if (patientAddress === doctorAddress) {
    console.log(
      `Resultado: true (patient === doctor — acesso sempre garantido pela política do pallet)`,
    );

    return true;
  }

  try {
    const stored = await api.query?.medicalPermissions?.permissions?.(
      patientAddress,
      doctorAddress,
    );

    const hasAccess = stored.toJSON() as boolean;

    console.log(
      `O médico ${doctorAddress} tem acesso ao paciente ${patientAddress}? -> ${hasAccess ? 'Sim' : 'Não'}`,
    );

    return hasAccess;
  } catch (err) {
    console.log('Error checking access:', err);
    return false;
  }
}

export async function grantAccess({
  patientAddress,
  doctorAddress,
}: Readonly<GrantAccess>): Promise<boolean | string> {
  // Keyring e conta originadora (paciente = Alice)
  const patientKeyringPair = keyring.addFromAddress(patientAddress);
  const doctorKeyringPair = keyring.addFromAddress(doctorAddress);

  const extrinsic = api.tx.medicalPermissions.grantAccess(
    doctorKeyringPair.address,
  );

  // this call fires up the authorization popup
  // await web3Enable('HealthChain POC UI');
  // const injector = await web3FromSource(patientKeyringPair.meta!.source!);

  const { promise, reject, resolve } = withResolvers<boolean | string>();

  try {
    const unsubscribe = await extrinsic.signAndSend(
      patientKeyringPair.address,
      // { signer: injector.signer },
      ({ status, events, dispatchError }: SubmittableResult) => {
        console.log('Status:', status.type);

        if (status.isInBlock) {
          console.log('Incluído no bloco:', status.asInBlock.toHex());
        }

        resolve(true);

        // // Erro de dispatch
        // if (dispatchError) {
        //   if ((dispatchError as any).isModule) {
        //     // Decodifica erro do runtime
        //     try {
        //       const decoded = api.registry.findMetaError(
        //         (dispatchError as any).asModule,
        //       );
        //       const { section, name, docs } = decoded;
        //       console.error(
        //         `Erro do runtime: ${section}.${name} — ${docs.join(' ')}`,
        //       );
        //     } catch (e) {
        //       console.error('Erro ao decodificar dispatchError do módulo:', e);
        //     }
        //   } else {
        //     console.error('Erro:', dispatchError.toString());
        //   }
        // }

        // // Eventos
        // if (events && events.length) {
        //   events.forEach(({ event: { section, method, data }, phase }: any) => {
        //     console.log(
        //       `Event: ${section}.${method} (phase=${phase.toString()}) ->`,
        //       data.toString(),
        //     );
        //   });
        // }

        // if (status.isFinalized) {
        //   console.log('Finalizado no bloco:', status.asFinalized.toHex());
        //   // unsub é função de unsubscribe
        //   try {
        //     if (typeof unsubscribe === 'function') {
        //       unsubscribe();
        //     } else if (
        //       unsubscribe &&
        //       typeof (unsubscribe as any).unsubscribe === 'function'
        //     ) {
        //       // fallback caso runtime/versão retorne um objeto
        //       (unsubscribe as any).unsubscribe();
        //     }
        //   } catch (e) {
        //     // ignore
        //   }
        //   api.disconnect().catch(() => {});
        //   process.exit(0);
        // }
      },
    );
    console.log('🚀 ~ grantAccess ~ unsubscribe:', unsubscribe);

    // Alguns ambientes/versões retornam hash ou objeto; faça uma checagem segura
    // if (unsubscribe && typeof (unsubscribe as any).toHex === 'function') {
    //   console.log('Hash da transação:', (unsubscribe as any).toHex());
    // }
  } catch (err) {
    console.error('Erro ao enviar extrinsic:', err);
    reject((err as Error).message);
  }

  return promise;
}
