# 🚀 Substrate Key-Value Chain
A simple Substrate-based solo blockchain demonstrating key-value on-chain storage (account_id → username) with a custom JSON-RPC interface and a two-node setup.

## 🧱 Prerequisites
Make sure you have the following installed:

- Rust
- subkey (for key generation and insertion)
- cargo, git
- Linux/macOS or WSL2 (Windows Subsystem for Linux)
- Optional: jq, curl


## 🏗️ Build the Chain
Clone the project and build it:

```bash
git https://github.com/its-saeed/terminal-3.git
cd terminal-3
cargo build --release
```

## 🔗 Run the Chain

### 🗝️ Step 1: Insert Keys for Authority Nodes
You need to add keys so each node can act as a block-producing authority.

Generate a key (optional)
You can use a known development key or generate one:

```bash
subkey generate --scheme sr25519
```

For simplicity, let’s use Alice and Bob (built-in dev keys).

```bash
./target/release/solochain-template-node key insert \
  --base-path /tmp/nodeA \
  --chain local \
  --scheme sr25519 \
  --suri //Alice \
  --key-type aura
```
Insert Bob’s key (Node B)
```bash
./target/release/solochain-template-node key insert \
  --base-path /tmp/nodeB \
  --chain local \
  --scheme sr25519 \
  --suri //Bob \
  --key-type aura
```

### 🏃 Step 2: Run Two Nodes

Terminal 1: Start Node A

```bash
./target/release/solochain-template-node \
  --base-path /tmp/nodeA \
  --chain local \
  --port 30333 \
  --rpc-port 9944 \
  --validator \
  --name nodeA
```

If you get an error like this:

```bash
Error: NetworkKeyNotFound("/tmp/nodeA/chains/local_testnet/network/secret_ed25519")
```
You need to create a key for the network. Run this command:

```bash
./target/release/solochain-template-node key generate-node-key --base-path /tmp/nodeA
```

Terminal 2: Start Node B and connect it to Node A

```bash
./target/release/solochain-template-node \
  --base-path /tmp/nodeB \
  --chain local \
  --port 30334 \
  --rpc-port 9945 \
  --validator \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/<NODE_A_PEER_ID> \
  --name nodeB
```

🔍 To find <NODE_A_PEER_ID>, check the logs of Node A:

```log
2025-05-13 16:22:42.642  INFO sub-libp2p: Local node identity is: 12D3KooW...
Replace that in the --bootnodes parameter.
```

### 🔎 Verifying That It Works
If both nodes are connected, you’ll see block production logs like this on both terminals:

```log
⛓️  Imported #10 (0xabc...) Hash: 0xabc...
Also check peer count in logs:
```

```log
2025-05-13 16:24:00 💤 Idle (1 peers), best: #10
📡 JSON-RPC API Access
```

Now that both nodes are running:

Node A JSON-RPC: http://127.0.0.1:9933

Node B JSON-RPC: http://127.0.0.1:9934

You can interact with either using curl, Postman, or frontend apps like Polkadot.js Apps.

## 🔑 Setting and Querying account_id → username
Now let’s see how to store and fetch a username for an account using JavaScript and Polkadot.js API.

### Store account_id → username on the Chain
This script allows an account to set its username in storage:

```javascript
import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';

const wsProvider = new WsProvider('ws://127.0.0.1:9944'); // Connect to Node B
const api = await ApiPromise.create({ provider: wsProvider });

async function main() {
    // Create a keyring instance
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    console.log(`Alice's account ID: ${alice.address}`);

    // Send a signed transaction
    const tx = api.tx.template.setUsername('AliceUsername');
    const hash = await tx.signAndSend(alice);

    console.log(`Transaction hash: ${hash}`);
}

main().then(() => {
    console.log('Username stored successfully!');
});
```

Retrieve the Stored Username
To query a stored username from the chain:
```javascript
async function getUsername(account) {
    const username = await api.query.template.usernames(account);
    console.log(`Stored username: ${username.toHuman()}`);
}

getUsername('5Gw3s7q4QLkSWwknzttLBvDqFf1uRnXcVWuJxNCEgFh5cKZT'); // Replace with Alice's Account ID
```


