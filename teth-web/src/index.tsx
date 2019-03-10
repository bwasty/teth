import * as React from 'react'
import { Component } from 'react'
import { render } from 'react-dom'

import { Account } from 'web3-eth-accounts';
import light, { balanceOf$ } from '@parity/light.js';

import { provider, web3 } from './provider';
import { WebsocketProvider } from 'web3-providers/types';

light.setProvider(provider);

interface AppState {
  account: Account,
  balance: string,
  topAccounts: any,
  latestBlocks: any[],
}

class App extends Component<{}, AppState> {
  constructor(props) {
    super(props)

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

    this.state = { 
      account, 
      balance: "0", 
      topAccounts: [],
      latestBlocks: [],
    };

    // TODO!!: doesn't work when using Web3.js as provider
    // balanceOf$(account.address).subscribe(balance => {
    //   // this.setState(state => {return {balance, ...state} })
    //   console.log(balance)
    //   this.setState({ balance: balance.toString() })
    // })
    web3.eth.getBalance(account.address)
      .then((balance) => this.setState({ balance: web3.utils.fromWei(balance)}))

    const provider = web3.currentProvider as WebsocketProvider;
    provider.send('teth_topAccounts', []).then((topAccounts: string[]) => {
      this.setState({ topAccounts: topAccounts.map(([address, balance]) => {
        return {address: address, balance: web3.utils.fromWei(balance)}
      })});
    })

    web3.eth.getBlock('latest').then(block => {
      this.setState({latestBlocks: [block]})
    })

    // TODO!: HACK for updating connection string...
    setInterval(() => this.forceUpdate(), 2000);
  }
  connectionString() {
    let provider = web3.currentProvider as WebsocketProvider;
    if (provider.isConnecting()) {
      return ""
    }
    if (provider.connected) {
      return <i style={{float: 'right', fontSize: 'smaller', color: 'green'}}>Connected to {provider.host}</i>
    }
    return <i style={{float: 'right', fontSize: 'smaller', color: 'red'}}>Disconnected</i>
  }

  render() {
    return (
      <div>
        {this.connectionString()}
        <b>Address:</b> {this.state.account.address} 
        <span style={{color: "gray", fontSize: "smaller"}}>(generated in-memory wallet)</span>
        <br />
        <b>Balance:</b> {this.state.balance} TETH
        <hr />
        <b>Top 5 Accounts:</b>
        <br />
        {this.state.topAccounts.map(({address, balance}) =>
          <div key={address}>{address} - {balance} TETH</div>)}
        <hr />
        <b>Latest block(s):</b>
        {this.state.latestBlocks.map(block => {
          let date = new Date(block.timestamp * 1000)
          return <div key={block.number}>
            #{block.number} - {block.transactions.length} transactions - {date.toLocaleDateString()} {date.toLocaleTimeString()}
          </div>
        })}
        <br />
      </div>
    )
  }
}

render(<App />, document.getElementById('root'))
