# ZenLayer

A development template for quick building EVM compliant L2 for the BoolNetwork ecosystem.

## Run

Build the project

```bash
cargo build --release
```

Run the nodes

```bash
./nodes.sh
```

## Usage

If you customize the chain, request the modification content of the `chain_spec.rs` file, including:

* Token name and Total number of tokens issued.
* EVMChainId (Default dev: 17977, testnet: 17978, mainnet: 17979).
* Sudo account and POA account.
