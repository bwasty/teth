# teth - Toy Ethereum Client 
[![Actions Status](https://wdp9fww0r9.execute-api.us-west-2.amazonaws.com/production/badge/bwasty/teth?style=flat)](https://github.com/bwasty/teth/actions) [![](https://tokei.rs/b1/github/bwasty/teth)](https://github.com/Aaronepower/tokei)

## Goals
* implement Ethereum straight from the [Yellow Paper](https://github.com/ethereum/yellowpaper/)
* (over)simplify things for a short and understandable implementation
  - compatibility with real Ethereum: maybe later

## Current state
Reading the Yellow Paper and stubbing some structs...

## Difference to parity-ethereum
`teth` uses some of the same util crates as `parity-ethereum` (such as `rlp`, `patricia-trie`, `tiny-keccak` and `ethereum-types`), but otherwise aims to be an independent implementation of the specification. Furthermore, `parity-ethereum` is quite large (~143k LoC) and has many options; `teth` aims to remain small and just implement the specification in a straight-forward manner.

## Development

### Coverage
Install [cargo-cov](https://github.com/kennytm/cov) and run `./test_with_coverage.sh` (opens HTML report when done). Requires nightly Rust.
