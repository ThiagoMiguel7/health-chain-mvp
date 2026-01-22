// `dot` is the identifier assigned when executing `npx papi add`
import { healthchain } from '@polkadot-api/descriptors';
import { createClient } from 'polkadot-api';
// Use this import for Node.js environments
//import { getWsProvider } from 'polkadot-api/ws-provider/web';
import { getWsProvider } from 'polkadot-api/ws-provider/node';
import { withPolkadotSdkCompat } from 'polkadot-api/polkadot-sdk-compat';


// Signer utilities
import { cryptoWaitReady } from '@polkadot/util-crypto';
import { Keyring } from '@polkadot/keyring';
import { getPolkadotSigner } from 'polkadot-api/signer';

// Helpers for address argument shapes
import { MultiAddress } from '@polkadot-api/descriptors';

//------------------------------------------------------------

const WS_ENDPOINT = 'ws://127.0.0.1:9944';

// seed/mnemonic da conta que vai pagar a transação.
const SENDER_MNEMONIC = process.env.SENDER_MNEMONIC || 'INSERT_YOUR_MNEMONIC_HERE';

// Endereço do médico que receberá o acesso (destino).
const DOCTOR_ADDRESS = process.env.DOCTOR_ADDRESS || '5...'; // substitua

//-------------------------------------------------------------


// Establish a connection to the healthchain
const client = createClient(
    // The Polkadot SDK nodes may have compatibility issues; using this enhancer is recommended.
    // Refer to the Requirements page for additional details
    withPolkadotSdkCompat(getWsProvider('ws://127.0.0.1:9944')),
);

// To interact with the chain, obtain the `TypedApi`, which provides
// the types for all available calls in that chain
const healthchainApi = client.getTypedApi(healthchain);




