// Required imports
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { u8aToNumber } = require('@polkadot/util');

async function main () {
  // Initialise the provider to connect to the local node
  const provider = new WsProvider('ws://127.0.0.1:9944');

  // Create the API and wait until ready
  const api = await ApiPromise.create({ provider });

  // the hex: `node-template::storage::[block#]`
  await api.rpc.offchain.localStorageGet('PERSISTENT', '0x6e6f64652d74656d706c6174653a3a73746f726167653a3a08000000', (option) => {
    if (option.isSome) {
        console.log(u8aToNumber(option.unwrap()) == 1234);
    } else {
        console.log('no record found')
    }

    process.exit()
  });
}

main().catch(console.error).finally();