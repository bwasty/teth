import Web3 from 'web3';
const web3 = new Web3("ws://localhost:8546");
(window as any).web3 = web3; // for debugging

(async () => {
    // await provider.send('teth_faucet', [account.address]);

    let tx = {
        nonce: "0",
        chainId: "85",
        to: "0x96F1e2BdcB7645773D3DE58BcCB6223c44fA7D29",
        value: "42000", 
        gasPrice: "2",
        gas: "42000",
    }
    // TODO!!: Cannot read property 'sign' of undefined
    let signedTx = await web3.eth.accounts.signTransaction(tx, account.privateKey);
})()

// const wallet = web3.eth.accounts.wallet;
// wallet.create(1)
// wallet.save('teth')

// import { AbstractMethod } from 'web3-core-method';
// class TopAccountsMethod extends AbstractMethod {
//     constructor(utils, formatters) {
//         super('teth_topAccounts', 2, utils, formatters);
//     }
// }

// (web3.eth as any).methodFactory.methods.topAccounts = TopAccountsMethod;


