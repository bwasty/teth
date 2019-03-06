import Web3 from 'web3';
import { Account } from 'web3-eth-accounts';
const web3 = new Web3("ws://localhost:8546");
(window as any).web3 = web3; // for debugging

// TODO!!: use wallet instead...
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

document.body.append(`Address: ${account.address}`)

web3.eth.getBalance(account.address)
