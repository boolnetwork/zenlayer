#!/bin/bash

# create logs directory
mkdir logs

# start node
RUST_LOG=info nohup ./target/release/zenlayer \
  --base-path ./base/alice \
  --chain testnet_local \
  --port 30333 \
  --rpc-port 9933 \
  --rpc-cors all \
  --node-key 1200000000000000000000000000000000000000000000000000000000001111 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --enable-offchain-indexing true \
  --blocks-pruning=archive \
  --state-pruning=archive \
  --ethapi="debug,trace,txpool" \
  --validator \
  >> ./logs/node_alice.log &

sleep 2
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d '{"jsonrpc":"2.0","id":1,"method":"author_insertKey","params": ["aura","0x19b8720a21158be6333aab6bbf64b23876b1259e158755c88009edf1381a49fa","0xd85d1a3e8b82ae2f9dae87d707a29927556bfb115c3c2447e81f189b18ca7641"]}'
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d '{"jsonrpc":"2.0","id":1,"method":"author_insertKey","params": ["gran","0xada3b9b4ed2f2532850072056c2d891bd160d4d107451052d00b7e5bf339fac5","0x2730419ee3b96fe8c0183812618b1e6ba307e214357296d0f3093bc9b8eea882"]}'

RUST_LOG=info nohup ./target/release/zenlayer \
  --base-path ./base/bob \
  --chain testnet_local \
  --port 30334 \
  --rpc-port 9934 \
  --rpc-cors all \
  --node-key 1200000000000000000000000000000000000000000000000000000000002222 \
  --validator \
  --enable-offchain-indexing true \
  --blocks-pruning=archive \
  --state-pruning=archive \
  --ethapi="debug,trace,txpool" \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWQPWtMXFthzk4jyywVZ7WdNyMP8myS29jSchhAaCgKrt2 \
  >> ./logs/node_bob.log &

sleep 2

curl http://localhost:9934 -H "Content-Type:application/json;charset=utf-8" -d '{"jsonrpc":"2.0","id":1,"method":"author_insertKey","params": ["aura","0x14fa9e0103a854102fa45feebf643df5d93942e4946f1ee50358d17e3ecc889d","0xfafaa9ad6290225a21d8a84bb8fca99ba8347ae3758f4ecfa4fe453103137f3b"]}'
curl http://localhost:9934 -H "Content-Type:application/json;charset=utf-8" -d '{"jsonrpc":"2.0","id":1,"method":"author_insertKey","params": ["gran","0xce9c5e04ceea64d9c82e123e1f7feeea5ac79420e3306f052f6a6fcb7bacae44","0xa19d3fcda429cad5add5eb939c474d8c8080f6a78f0d32fd926040d331dbf81b"]}'

RUST_LOG=info nohup ./target/release/zenlayer \
  --base-path ./base/charlie \
  --chain testnet_local \
  --port 30335 \
  --rpc-port 9935 \
  --rpc-cors all \
  --node-key 1200000000000000000000000000000000000000000000000000000000003333 \
  --validator \
  --enable-offchain-indexing true \
  --blocks-pruning=archive \
  --state-pruning=archive \
  --ethapi="debug,trace,txpool" \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWQPWtMXFthzk4jyywVZ7WdNyMP8myS29jSchhAaCgKrt2 \
  >> ./logs/node_charlie.log &

sleep 2

curl http://localhost:9935 -H "Content-Type:application/json;charset=utf-8" -d '{"jsonrpc":"2.0","id":1,"method":"author_insertKey","params": ["aura","0x0e397595fd3b346136439dccb061890a1328557dccafde1095c3c9374f7f1892","0x28c0a26d79ec0fe47dc7aa6001c33a877b9cad6779953912d164353a5611d342"]}'
curl http://localhost:9935 -H "Content-Type:application/json;charset=utf-8" -d '{"jsonrpc":"2.0","id":1,"method":"author_insertKey","params": ["gran","0x0f175cbd49ba512094fa901c305e1343ad6c7858d2c7753b84bb3e4e1f52574a","0x5a8c4ca176d24805b258a8b491289a02bb3ac9a9284e7358096e8cf12e7a87a7"]}'

sleep 4
pkill zenlayer
sleep 4

RUST_LOG=info nohup ./target/release/zenlayer \
  --base-path ./base/alice \
  --chain testnet_local \
  --port 30333 \
  --rpc-port 9933 \
  --rpc-cors all \
  --node-key 1200000000000000000000000000000000000000000000000000000000001111 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --enable-offchain-indexing true \
  --blocks-pruning=archive \
  --state-pruning=archive \
  --ethapi="debug,trace,txpool" \
  --validator \
  >> ./logs/node_alice.log &

sleep 2

RUST_LOG=info nohup ./target/release/zenlayer \
  --base-path ./base/bob \
  --chain testnet_local \
  --port 30334 \
  --rpc-port 9934 \
  --rpc-cors all \
  --node-key 1200000000000000000000000000000000000000000000000000000000002222 \
  --validator \
  --enable-offchain-indexing true \
  --blocks-pruning=archive \
  --state-pruning=archive \
  --ethapi="debug,trace,txpool" \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWQPWtMXFthzk4jyywVZ7WdNyMP8myS29jSchhAaCgKrt2 \
  >> ./logs/node_bob.log &

sleep 2

RUST_LOG=info nohup ./target/release/zenlayer \
  --base-path ./base/charlie \
  --chain testnet_local \
  --port 30335 \
  --rpc-port 9935 \
  --rpc-cors all \
  --node-key 1200000000000000000000000000000000000000000000000000000000003333 \
  --validator \
  --enable-offchain-indexing true \
  --blocks-pruning=archive \
  --state-pruning=archive \
  --ethapi="debug,trace,txpool" \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWQPWtMXFthzk4jyywVZ7WdNyMP8myS29jSchhAaCgKrt2 \
  >> ./logs/node_charlie.log &
