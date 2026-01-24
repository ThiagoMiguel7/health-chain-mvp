import { ApiPromise, WsProvider } from '@polkadot/api';
import { SubmittableExtrinsic } from '@polkadot/api/types';

import { Keyring } from '@polkadot/keyring';
import { KeyringPair } from '@polkadot/keyring/types';

import { WS_URL } from './shared';
import { withResolvers } from './promises';

const provider = new WsProvider(WS_URL);
const api = await ApiPromise.create({ provider });

const keyring = new Keyring({ type: 'sr25519', ss58Format: 2 });

type BlockchainActionProps = { patientAddress: string; doctorAddress: string };
type BlockchainCreateRecordProps = BlockchainActionProps & {
  fileHashHex: string;
};

export type BlockchainActionResult =
  | {
      success: true;
      blockNumber: number;
      transactionHash: string;
    }
  | {
      success: false;
      error: Error;
    };

keyring.addFromUri('//Alice');
keyring.addFromUri('//Bob');
keyring.addFromUri('//Charlie');
keyring.addFromUri('//Dave');
keyring.addFromUri('//Eve');
keyring.addFromUri('//Ferdie');

export async function checkAccess({
  patientAddress,
  doctorAddress,
}: Readonly<BlockchainActionProps>): Promise<boolean> {
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

async function submitExtrinsic(
  extrinsic: SubmittableExtrinsic<'promise'>,
  signer: KeyringPair,
): Promise<BlockchainActionResult> {
  const { promise, resolve, reject } = withResolvers<BlockchainActionResult>();

  let unsubscribe: (() => void) | undefined;
  let completed = false;

  const finish = (result?: BlockchainActionResult, error?: string) => {
    if (completed) return;
    completed = true;
    unsubscribe?.();

    if (error) {
      reject({ success: false, error: new Error(error) });
    } else if (result) {
      resolve(result);
    }
  };

  try {
    unsubscribe = await extrinsic.signAndSend(
      signer,
      ({ status, events, dispatchError }) => {
        if (completed) return;

        // ❌ Dispatch error
        if (dispatchError) {
          if (dispatchError.isModule) {
            const meta = api.registry.findMetaError(dispatchError.asModule);
            return finish(
              undefined,
              `${meta.section}.${meta.name}: ${meta.docs.join(' ')}`,
            );
          }

          return finish(undefined, dispatchError.toString());
        }

        const success = events.some(
          ({ event }) =>
            event.section === 'system' && event.method === 'ExtrinsicSuccess',
        );

        const failed = events.some(
          ({ event }) =>
            event.section === 'system' && event.method === 'ExtrinsicFailed',
        );

        if (failed && !success) {
          return finish(undefined, 'Extrinsic finalizada com falha');
        }

        if (success) {
          const blockHash = status.asInBlock;
          api.rpc.chain.getBlock(blockHash).then(signedBlock => {
            const blockNumber = signedBlock.block.header.number.toNumber();
            console.log(`Block number: ${blockNumber}`);

            finish({
              blockNumber,
              success: true,
              transactionHash: extrinsic.hash.toHex(),
            });
          });
        }
      },
    );
  } catch (err) {
    finish(undefined, (err as Error).message);
  }

  return promise;
}

export async function grantAccess({
  patientAddress,
  doctorAddress,
}: Readonly<BlockchainActionProps>): Promise<BlockchainActionResult> {
  const patient = keyring.getPair(patientAddress);

  const extrinsic = api.tx.medicalPermissions.grantAccess(doctorAddress);

  return submitExtrinsic(extrinsic, patient);
}

export async function revokeAccess({
  patientAddress,
  doctorAddress,
}: Readonly<BlockchainActionProps>): Promise<BlockchainActionResult> {
  const patient = keyring.getPair(patientAddress);

  const extrinsic = api.tx.medicalPermissions.revokeAccess(doctorAddress);

  return submitExtrinsic(extrinsic, patient);
}

export async function createRecord({
  patientAddress,
  doctorAddress,
  fileHashHex,
}: Readonly<BlockchainCreateRecordProps>): Promise<BlockchainActionResult> {
  const doctor = keyring.getPair(doctorAddress);

  const extrinsic = api.tx.medicalHistory.createRecord(
    patientAddress,
    fileHashHex,
  );

  return submitExtrinsic(extrinsic, doctor);
}
