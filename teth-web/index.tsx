import * as React from 'react'
import { Component } from 'react'
import { render } from 'react-dom'

class App extends Component {
  render() {
    return (<h1>Hello World!</h1>)
  }
}

render(<App />, document.getElementById('app'))
