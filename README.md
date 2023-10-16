# Sorobounty.xyz
![Image](https://github.com/Tellus-Cooperative/sorobounty.xyz/blob/main/sorobounty_logo.png)
## Overview
The Bounty Hunter Web Application is a platform that facilitates the creation and participation in bounties for various tasks. This application leverages the Stellar blockchain through the Soroban platform to establish an Escrow Smart Contract, ensuring secure fund management, authentication, and a review stage for accepting or rejecting work.

## How to execute on local environment
Download Project:
-------------
```
$ git clone https://github.com/alexanderkoh/escrow-soroban
```

Build & Deploy Smart Contract:
-------------
```
$ cd escrow-soroban\contracts
$ soroban contract build
$ soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/soroban_token_contract.wasm \
    --source <admin> \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022'
```
As a result, contract-id is generated

To generate binding module:
```
$ soroban contract bindings typescript \
	--wasm target/wasm32-unknown-unknown/release/soroban_escrow_smart_contract.wasm \
	--output-dir ../frontend_vite/bountyhunter_module \
	--contract-id <contract-id> \
	--rpc-url https://rpc-futurenet.stellar.org \
	--network-passphrase 'Test SDF Future Network ; October 2022'
```

Run BackEnd:
-------------
On a new terminal:
```
$ cd escrow-soroban\backend
$ npm install
$ npm run start
```

Run FrontEnd:
-------------
On a new terminal:
```
$ cd escrow-soroban\frontend
# yarn
$ npm run start
```

Access to the following URL:
-------------
https://localhost:5173/
