// import Api from '@parity/api';
import Web3 from 'web3';

let protocol = 'ws';
let port = 8546;
if (location.protocol === 'https:') {
  protocol = 'wss'
  port = 8547 // proxied via caddy
}

let wsUrl = `${protocol}://${location.hostname}:${port}`

let win = window as any;
let web3 = new Web3(wsUrl);
win.web3 = web3;

// const provider = win.web3
//   ? win.web3.currentProvider
//   : new Api.Provider.Ws(wsUrl);
const provider = win.web3.currentProvider

export {provider, web3};
