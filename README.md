# canvas-node

Node implementation for Canvas, a Substrate chain for smart contracts.

To be continued....

To run local dev node, do

```
cargo run --release -- --dev
```

To run test net 1, do

```
cargo run --release
```

or

```
cargo run --release -- --chain=./res/testnet-1.json
```

# polkadot.js.org custom type

```json
{
  "Address": "AccountId",
  "LookupSource": "AccountId",
  "RefCount": "u8",
  "AssetId": "u32",
  "Name": "[u8;16]",
  "Symbol": "[u8;8]",
  "AssetInfo": {
    "name": "Name",
    "symbol": "Symbol",
    "decimals": "u8"
  },
  "ExchangeId": "u32",
  "Id": "u32",
  "TokenBalance": "u64",
  "Exchange": {
    "token_id": "AssetId",
    "liquidity_id": "AssetId",
    "account": "AccountId"
  },
  "SwapHandleOf": {
    "_enum": {
      "ExchangeId": "(ExchangeId)",
      "AssetId": "(AssetId)"
    }
  }
}
```
