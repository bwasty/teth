import * as React from 'react'
import { Component } from 'react'
import { render } from 'react-dom'

import { Account } from 'web3-eth-accounts';
// import light, { balanceOf$ } from '@parity/light.js';

import { provider, web3 } from './provider';
import { WebsocketProvider } from 'web3-providers/types';

// light.setProvider(provider);

interface AppState {
  account: Account,
  balance: number,
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
      balance: null, 
      topAccounts: [],
      latestBlocks: [],
    };

    // TODO!!: doesn't work when using Web3.js as provider
    // balanceOf$(account.address).subscribe(balance => {
    //   // this.setState(state => {return {balance, ...state} })
    //   console.log(balance)
    //   this.setState({ balance: balance.toString() })
    // })
    this.updateBalance();

    this.updateTopAccounts();

    web3.eth.getBlock('latest').then(block => {
      this.setState({latestBlocks: [block]})
      if (block.number === 0) { return };
      for (let i = block.number - 1; i >= Math.max(0, block.number - 6); i--) {
        web3.eth.getBlock(i).then(block => {
          this.setState({latestBlocks: [...this.state.latestBlocks, block]})
        });
      }
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
  async updateBalance() {
    let balance = await web3.eth.getBalance(this.state.account.address)
    this.setState({ balance: web3.utils.fromWei(balance) as any})
  }
  async updateTopAccounts() {
    const provider = web3.currentProvider as WebsocketProvider;
    let topAccounts = await provider.send('teth_topAccounts', []) as string[]
    this.setState({ topAccounts: topAccounts.map(([address, balance]) => {
      return {address: address, balance: web3.utils.fromWei(balance)}
    })});
  }
  faucetClick = async () => {
    await web3.eth.currentProvider.send('teth_faucet', [this.state.account.address]);
    this.updateBalance();
    this.updateTopAccounts();
  }
  render() {
    return (
      <div>
        {this.connectionString()}
        <b>Address:</b> {this.state.account.address} 
        <span style={{color: "gray", fontSize: "smaller"}}>(generated in-memory wallet)</span>
        <br />
        <b>Balance:</b> {this.state.balance} {this.state.balance !== null && 'TETH'}
        <br />
        <b>Faucet:</b>
        <button onClick={this.faucetClick} disabled={this.state.balance > 0}>Request 1 TETH</button>
        <span style={{color: "gray", fontSize: "smaller"}}>(only works for empty accounts)</span>
        <hr />
        <b>Top 5 Accounts:</b>
        <br />
        <table className="table is-striped">
        <tbody>
        {this.state.topAccounts.map(({address, balance}) =>
          <tr key={address}>
            <td>{address}</td><td align="right">{balance} TETH</td>
          </tr>)}
        </tbody>
        </table>
        <hr />
        <b>Latest block:</b>
        <table className="table is-striped">
        <tbody>
        {this.state.latestBlocks.map(block => {
          let date = new Date(block.timestamp * 1000)
          return <tr key={block.number}>
            <td>#{block.number}</td>
            <td>{block.transactions ? block.transactions.length : 0} transaction(s)</td>
            <td>{date.toLocaleDateString()} {date.toLocaleTimeString()}</td>
          </tr>
        })}
        </tbody>
        </table>
        <br />
      </div>
    )
  }
}

render(<App />, document.getElementById('root'))
