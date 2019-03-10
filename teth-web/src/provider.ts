import Api from '@parity/api';
import Web3 from 'web3';

let win = window as any;
let web3 = new Web3(`ws://${location.hostname}:8546`);
win.web3 = web3;

const provider = win.web3
  ? win.web3.currentProvider
  : new Api.Provider.Ws(`ws://${location.hostname}:8546`);

export {provider, web3};
