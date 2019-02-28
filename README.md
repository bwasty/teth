# teth - Toy Ethereum Client
[![Docs](https://img.shields.io/badge/docs-github.io-informational.svg)](https://bwasty.github.io/teth/teth/index.html)
[![Actions Status](https://wdp9fww0r9.execute-api.us-west-2.amazonaws.com/production/badge/bwasty/teth?style=flat)](https://github.com/bwasty/teth/actions) [![](https://tokei.rs/b1/github/bwasty/teth)](https://github.com/Aaronepower/tokei) [![](https://tokei.rs/b1/github/bwasty/teth?category=comments)](https://github.com/Aaronepower/tokei) 

## Goals
* implement Ethereum straight from the [Yellow Paper](https://github.com/ethereum/yellowpaper/)
* (over)simplify things at first for a quick implementation
  - no EVM/real contracts for now
  - compatibility with real Ethereum: maybe later if at all
* create a small browser game on top of it to teach the basics of Ethereum

## Current state
There are structs that should cover most necessary data structures, and a bit of validatation and execution logic here and there. 

Still reading the Yellow Paper...

## Difference to parity-ethereum
`teth` uses many of the same utility crates as `parity-ethereum` (such as `rlp`, `patricia-trie`, `tiny-keccak` and `ethereum-types`), but otherwise aims to be an independent implementation of the specification. Also, `parity-ethereum` is quite large (~143k LoC) and has many options; `teth` will remain small and just implement the specification in a straight-forward manner.

## Usage
_NOTE: Almost every command will fail with 'not implemented yet' at the moment!_

Run `teth` without arguments to see options and subcommands:
```
teth 0.0.1
A Toy Ethereum implementation.

USAGE:
    teth <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    node           Run a node.
    account        Query information about accounts.
    transaction    Query information about transactions.
    block          Query information about blocks.
    help           Prints this message or the help of the given subcommand(s)
```
`teth help node`:
```
USAGE:
    teth node [FLAGS]

FLAGS:
        --bootstrap    Bootstrap the chain (with the genesis block).
    -h, --help         Prints help information
```
`teth help account`:
```
USAGE:
    teth account <SUBCOMMAND>

FLAGS:
    -h, --help    Prints help information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    list    List accounts, ordered by balance (descending)
    show    Show details of account (balance etc.)
```


## Development

### Coverage
Install [cargo-cov](https://github.com/kennytm/cov) and run `./test_with_coverage.sh` (opens HTML report when done). Requires nightly Rust.
