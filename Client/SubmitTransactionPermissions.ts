

import { healthchain } from '@polkadot-api/descriptors';
import { createClient } from 'polkadot-api';
// Use this import for Node.js environments
import { getWsProvider } from 'polkadot-api/ws-provider/node';
import { withPolkadotSdkCompat } from 'polkadot-api/polkadot-sdk-compat';

// Signer utilities
import { cryptoWaitReady } from '@polkadot/util-crypto';
import { Keyring } from '@polkadot/keyring';
import { getPolkadotSigner } from 'polkadot-api/signer';

// Helpers for address argument shapes
import { MultiAddress } from '@polkadot-api/descriptors';

// ---- CONFIG - 
const WS_ENDPOINT = 'ws://127.0.0.1:9944';

// seed/mnemonic do conta que vai pagar a transação. Alice assinará a transação.
const SENDER_MNEMONIC = "bottom drive obey lake curtain smoke basket hold race lonely fit walk"; // Alice é a paciente.

// Endereço do médico que receberá o acesso (destino). Bob é o médico.
const DOCTOR_ADDRESS = "14E5nqKAp3oAJcmzgZhUD2RcptBeUBScxKHgJKU4HPNcKVf3";

// ------------------------------


const client = createClient(
    withPolkadotSdkCompat(getWsProvider(WS_ENDPOINT)),
);


const healthchainApi = client.getTypedApi(healthchain);  // ATENÇÃO: está retornado conjunto vazio para pallets.

async function main() {
    try {
        // 1) prepare crypto + keypair
        await cryptoWaitReady();
        const keyring = new Keyring({ type: 'sr25519' });
        const sender = keyring.addFromMnemonic(SENDER_MNEMONIC);

        console.log('Sender address:', sender.address);

        // 2) (opcional) listar pallets disponíveis para confirmar o nome do pallet/extrinsic
        console.log('Available pallets (healthchainApi.tx keys):', Object.keys(healthchainApi.tx));
        // Se o pallet se chamar diferente, use esse nome. Ex: healthchainApi.tx.MyPallet.grant_access

        // 3) criar um signer compatível com PAPI
        const signer = getPolkadotSigner(
            sender.publicKey,
            'Sr25519',
            // função de assinatura (PAPI usará esse callback para assinar bytes)
            async (payload: Uint8Array) => {
                // keyring pair.sign espera o payload e retorna uma signature
                // observe que KeyringPair.sign retorna uma Signature (u8a)
                return sender.sign(payload);
            }
        );

        // 4) construir a transação
        // Aqui usamos a convenção `Pallet.call({ ... })`.
        // * Substitua `MedicalPermissions` pelo nome real do pallet caso seja outro.
        // * O parâmetro do extrinsic é o `doctor` (AccountId). Exemplo usando MultiAddress:
        const tx = (healthchainApi.tx as any).MedicalPermissions?.grant_access({
            doctor: MultiAddress.Id(DOCTOR_ADDRESS),
        });

        if (!tx) {
            throw new Error('Falha ao construir a tx. Verifique o nome do pallet (MedicalPermissions) em healthchainApi.tx.');
        }

        // 5) assinar e submeter (aguarda envio)
        console.log('Assinando e submetendo a transação (grant_access) ...');
        const { txHash } = await tx.signAndSubmit(signer);

        console.log(`Transaction submitted with hash: ${txHash}`);
    } catch (err) {
        console.error('Infelizmente,', err);
        process.exitCode = 1;
    } finally {
        // encerra a conexão do client
        try {
            await client.destroy();
        } catch (e) {
            // ignore
        }
    }
}

main();
