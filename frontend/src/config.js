const CONTRACT_NAME = window.__RUNTIME_CONFIG__.VUE_APP_CONTRACT_NAME || 'nft.testnet';

function getConfig(env) {
    switch(env) {
        case 'development':
        case 'testnet':
            return {
                networkId: 'testnet',
                nodeUrl: 'https://rpc.testnet.near.org',
                contractName: CONTRACT_NAME,
                walletUrl: 'https://wallet.testnet.near.org',
                helperUrl: 'https://helper.testnet.near.org'
            };
        default:
            throw Error(`Unconfigured environment '${env}'. Can be configured in src/config.js.`);
    }
}

module.exports = getConfig;