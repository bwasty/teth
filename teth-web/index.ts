import Web3 from 'web3';
import { Account } from 'web3-eth-accounts';
const web3 = new Web3("ws://localhost:8546");
(window as any).web3 = web3; // for debugging

// TODO!: use wallet instead...after saving issue is fixed
let account: Account;
const storageKey = 'web3-local-account';
if (localStorage.getItem(storageKey)) {
    account = JSON.parse(localStorage.getItem(storageKey));
    console.log('restored account');
} else {
    account = web3.eth.accounts.create()
    localStorage.setItem('web3-local-account', JSON.stringify(account));
    console.log('created account');
}

(window as any).account = account;

let div = document.getElementById('app');
div.innerHTML = `Address: ${account.address}, `;

(async () => {
    let balance = await web3.eth.getBalance(account.address);
    div.append(`\nBalance: ${balance} Wei`)
})()

// const wallet = web3.eth.accounts.wallet;
// wallet.create(1)
// wallet.save('teth')
