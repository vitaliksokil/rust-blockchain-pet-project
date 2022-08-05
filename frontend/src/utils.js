import getConfig from './config.js';

import NearWalletSelector from "@near-wallet-selector/core";
import {setupNearWallet} from "@near-wallet-selector/near-wallet";
import {setupSender} from "@near-wallet-selector/sender";
import senderIcon from "./assets/sender-icon.png";
import nearWalletIcon from "./assets/near-wallet-icon.png";
import {providers} from "near-api-js";

const nearConfig = getConfig(window.__RUNTIME_CONFIG__.VUE_APP_NEAR_ENV || 'testnet');

async function init() {

    const walletSelector = window.walletSelector = await NearWalletSelector.init({
        network: "testnet",
        contractId: nearConfig.contractName,
        wallets: [
            setupNearWallet({iconUrl: nearWalletIcon}),
            setupSender({iconUrl: senderIcon}),
        ],
    });

    window.provider = new providers.JsonRpcProvider({
        url: walletSelector.network.nodeUrl,
    });


    await updateNearAcc();

}

async function updateNearAcc() {
    let currentUser = {
        accountId: '',
        balance: 0,
        isSignedIn: false,
    };
    const setCurrentUser = async () => {
        let accounts = await window.walletSelector.getAccounts();
        if (Array.isArray(accounts) && accounts.length) {
            let accountId = accounts[0].accountId;
            let result = await window.provider.query({
                request_type: "view_account",
                finality: "final",
                account_id: accountId,
            });
            if (result) {
                currentUser = {
                    // Gets the accountId as a string
                    accountId: accountId,
                    // Gets the user's token balance
                    balance: result.amount,
                    isSignedIn: await isSignedIn(),
                };
            }
        }
    }
    await setCurrentUser();
    window.walletSelector.on("signIn", async () => {
        await setCurrentUser();
        if (currentUser) {
            document.location.reload();
        }
    });
    window.nearAccount = currentUser;
}

async function signIn() {
    window.walletSelector.show();
}

async function signOut() {
    await window.walletSelector.signOut().then(() => {
        document.location.reload();
    }).catch((err) => {
        alert(err);
    });
}

async function isSignedIn() {
    return window.walletSelector.isSignedIn();
}

async function getZooById(accountId){
    const argsBase64 = window.btoa(JSON.stringify({id: accountId}))
    let result = await window.provider.query({
        request_type: "call_function",
        account_id: window.walletSelector.getContractId(),
        method_name: "get_zoo_by_id",
        args_base64: argsBase64,
        finality: "optimistic",
    })
    return JSON.parse(Buffer.from(result.result).toString())
}


export {
    init,
    signIn,
    signOut,
    isSignedIn,
    getZooById,
}