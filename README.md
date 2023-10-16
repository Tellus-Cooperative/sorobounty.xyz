# Sorobounty.xyz
![Image](https://github.com/Tellus-Cooperative/sorobounty.xyz/blob/main/sorobounty_logo.png)
## Overview
The Bounty Hunter Web Application is a platform that facilitates the creation and participation in bounties for various tasks. This application leverages the Stellar blockchain through the Soroban platform to establish an Escrow Smart Contract, ensuring secure fund management, authentication, and a review stage for accepting or rejecting work.
## Getting Started
### Download Project
Clone the project repository from GitHub:

```bash
$ git clone https://github.com/Tellus-Cooperative/sorobounty.xyz
```

## Build & Deploy Smart Contract
Navigate to the contracts directory:

```bash
$ cd sorobounty.xyz/contracts
```

### Build and deploy the Smart Contract:

```bash
$ soroban contract build
$ soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/soroban_token_contract.wasm \
    --source <admin> \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022'
```

This process generates a unique contract ID.

### Generate Binding Module
To generate the binding module, use the following command, replacing **`<contract-id>`** with the contract ID generated in the previous step:

```bash

$ soroban contract bindings typescript \
    --wasm target/wasm32-unknown-unknown/release/soroban_escrow_smart_contract.wasm \
    --output-dir ../frontend_vite/bountyhunter_module \
    --contract-id <contract-id> \
    --rpc-url https://rpc-futurenet.stellar.org \
    --network-passphrase 'Test SDF Future Network ; October 2022'

```

### Run Backend
Open a new terminal and navigate to the backend directory:

```bash
$ cd sorobounty.xyz/backend
```

Install the required dependencies and start the backend server:

```bash
$ npm install
$ npm run start
```

### Run Frontend
Open another terminal and navigate to the frontend directory:

```bash
$ cd sorobounty.xyz/frontend
```

Install dependencies and start the frontend application:

```bash
$ yarn
$ npm run start
```

Access the BountyHunter Web Application at the following URL: https://localhost:5173/

You are now ready to explore and utilize the Sorobounty.xyz on your local environment.
