## Smart Contract Architecture
The Soroban Smart Contract, embedded within the Bounty Hunter Web Application, facilitates the creation and execution of bounties on the Stellar blockchain. It ensures secure fund management, authentication, and a transparent review process for accepting or rejecting work. This versatile contract is composed of six modules, including **`admin`**, **`fee`**, **`work`**, **`bounty`**, **`lib`** and **`test`**, each catering to specific responsibilities in the contract's operation. With the Soroban Smart Contract, users can create, fund, submit, approve, reject, cancel, and close bounties, making it a comprehensive solution for managing and automating reward-based tasks within the Stellar network.

The following diagram depicts the overall architecture of the smart contract:

![contract_arch_bounties.png](https://github.com/Tellus-Cooperative/sorobounty.xyz/blob/main/contracts/contract_architecture.png)

## Modules
This project is structured into six modules, each responsible for specific aspects of the smart contract functionality:

***admin*** : Admin module is in charge of setting/getting administrator. It also supports checking administrator right.

***fee*** : Fee module is in charge of setting/getting fee. It also supports checking fee functionality. Fee information includes fee rate and wallet.

***work*** : Work module is in charge of submitting work. It supports creating, getting & setting of work.

***bounty*** : Bounty module is similar to a real bounty. It supports creating, funding (put money in escrow), submitting work, approving & rejecting work, cancelling, and closing work.

***lib*** : Lib module contains export functions for web developers to use. These functions will be explained further in the documentation.

***test*** : Test module contains test functions for testing with Cargo. Currently, it includes four test cases, which will be discussed in more detail later.

## Data structures
Data structures are defined in **`storage_types.rs`** and include:

**1. Constants:** These include constants like FEE_DECIMALS, DEF_FEE_RATE, etc.

**2. Enums:** Enumerations such as BountyStatus, WorkStatus, DataKey and error codes are defined here.

**3. Structs:** Various structs like FeeInfo, BountyInfo, etc., are defined to structure data within the contract.


## Function descriptions (Lib Module)
The Lib module provides several essential functions for managing the Escrow Smart Contract:

- `set_fee`: Sets fee information with parameters for the environment (e), fee rate (in units of 1/10FEE_DECIMALS), and fee wallet address.
- `create_bounty`: Creates a new bounty with parameters for the creator's address, bounty name, reward amount, payment token address, and deaine.
- `fund_bounty`: Adds funds to a specific bounty by providing the creator's address and the bounty's ID.
- `participate_bounty`: Allows workers to apply for a bounty by specifying their address and the bounty ID.
- `submit_work`: Enables workers to submit their work for a specific bounty, providing their address, the bounty ID, and the work repository URL.
- `approve_work`: Allows the creator to approve a worker's submitted work by specifying the creator's address and the work ID.
- `reject_work`: Allows the creator to reject a worker's submitted work using the creator's address and the work ID.
- `cancel_bounty`: Enables the creator to cancel a bounty by providing the creator's address and the bounty ID.
- `close_bounty`: Closes an expired bounty, a function typically executed by a special account (administrator), with parameters for the administrator's address and the bounty ID.

## Build, Test, and Deploy

### Prerequisites
Before building, testing, or deploying the Soroban Escrow Smart Contract, ensure you have the following prerequisites:

- Install Rust using **`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`**.
- Add the **`wasm32-unknown-unknown`** target with **`rustup target add wasm32-unknown-unknown`**.
- Install Soroban CLI version **`0.8.0`** with **`cargo install --locked --version 0.8.0 soroban-cli`**.
- Install **`build-essential`** with **`sudo apt install build-essential`**.
- Make sure Soroban is properly set up.

### Tests
Run tests using the following commands:

```bash
cargo test
```

### Deployments
To deploy the Soroban Escrow Smart Contract, use the following commands:

```bash
soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/[project-name].wasm \
    --source <secret key of issuer> \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022'
```

A new contract ID will be generated upon successful deployment.
