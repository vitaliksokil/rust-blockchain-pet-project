'use strict';

const {connect, keyStores, KeyPair, utils} = require("near-api-js");
const {parseSeedPhrase, generateSeedPhrase} = require("near-seed-phrase");

exports.test = async function (req, res) {

    const privateKey = 'ed25519:PgkjZdHeVGmKNtcgJkvtzikMZEjw4fEbWgrivrJ1WQJ6gP1M4gEbGmxkjEb7AZnQWuReyFj7QcQk5HTAZuGbmzW';
    const keyPair = utils.KeyPair.fromString(privateKey);

    const keyStore = new keyStores.InMemoryKeyStore();
    await keyStore.setKey('testnet', 'uzoo.testnet', keyPair);

    const config = {
        keyStore,
        networkId: "testnet",
        nodeUrl: "https://rpc.testnet.near.org",
    };
    console.log(keyStore)
    const newAccountId = "near_wallet_from_program_7.testnet";
    const near = await connect({...config, keyStore});
    const creatorAccount = await near.account("uzoo.testnet");
    console.log(creatorAccount);
    const {seedPhrase, publicKey, keyPairNew} = generateSeedPhrase();

    // const keyPairNew = KeyPair.fromRandom("ed25519");
    // const publicKey = keyPairNew.publicKey.toString();

    // await keyStore.setKey(config.networkId, newAccountId, keyPairNew);
    console.log({seedPhrase, publicKey, keyPairNew});

    let result = await creatorAccount.functionCall({
        contractId: "testnet",
        methodName: "create_account",
        args: {
            new_account_id: newAccountId,
            new_public_key: publicKey,
        },
        // gas: "300000000000000",
        attachedDeposit: utils.format.parseNearAmount('0.01'),
    });

    res.json(result)
};

