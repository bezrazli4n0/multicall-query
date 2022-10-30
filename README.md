# multicall-query

## 🤔 Overview
Simple tool for using [multicall](https://dl.acm.org/doi/pdf/10.1145/3457337.3457839) operations on [ethereum](https://ethereum.org/)-based blockchains with ability to export results.

## 🛠 Building:
1. Install Rust.
2. Open your terminal.
3. Write: `cargo build --release`.

## ✨ Example usage:
`./multicall-query --url 'NODE_RPC_URL' --target-addrs-path fixtures/uniswap_v2_pairs_sample.txt --target-abi-path abi/uniswap_v2_pair.json --target-func-name getReserves`.