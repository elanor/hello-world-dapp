# 'Hello World' dApp Tutorial

## Introduction

This tutorial is designed for blockchain developers who are new to Concordium. It guides through the process of setting up the Concordium development environment and creating a "Hello World" decentralized application (dApp) on the Concordium blockchain.

## Why Concordium

Concordium offers a range of appealing features for blockchain developers:

- `Innovation in Security and Efficiency`: Concordium integrates advanced technologies such as Zero-Knowledge proofs and innovative consensus mechanisms, enhancing blockchain security and efficiency. The Concordium Research Center in Denmark continuously develops and incorporates these technologies into the platform.

- `Environmental Sustainability`: Concordium is one of the most energy-efficient blockchain companies, actively supporting projects aimed at mitigating climate change. This makes it a suitable choice for developers seeking to minimize the environmental impact of their blockchain applications.

- `Data Ownership and User Verification`: Concordium ensures users retain control over their data through built-in verification and authentication mechanisms, fostering trust while maintaining strong security protocols.

- `Community and Support`: Concordium offers access to a growing community of developers and provides support through grants awarded via decentralized voting, ensuring fair and equitable distribution of resources.

For developers transitioning from other blockchains, Concordium offers resources to facilitate the transition from platforms such as [Ethereum](https://developer.concordium.software/en/mainnet/smart-contracts/onboarding-guide-ethereum-developers/faq.html) and [Solana](https://developer.concordium.software/en/mainnet/smart-contracts/onboarding-guide-solana-developers/faq.html).

<!-- how you can explain a complex project to a developer who is new to Concordium, expecting them to know what blockchain is.  -->

## Setting up the environment

To set up the environment, three tools are required: Rust, Cargo Concordium and Concordium Client.  

1. Installing Rust
    - Rust is the language for smart contract development in Concordium. 
        For Unix-based systems (Linux/MacOS), the following command installs Rust:

        ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```

        For Windows, download and run `rustup-init.exe` and then follow installation instructions.

    - Once Rust is installed, the following tools become available:
        1. `rustc` - the Rust compiler
        2. `rustup` - update tool for Rust, including the Rust compiler
        3. `cargo` - a package manager for Rust

    - Concordium smart contracts use Web Assembly (Wasm) on-chain language. To use the Wasm language, install the Wasm target for compiling Rust contracts to Wasm, by running command: 

        ```rustup target add wasm32-unknown-unknown```

2. Installing Cargo Concordium

    `cargo-concordium` is a tool for developing smart contracts for Concordium blockchain. It is used for compiling and testing smart contracts, and enables features such as building contract schemas. For Cargo Concordium installation use command

	```cargo install --locked cargo-concordium```

    To see all the subcommands available, run:

    ```cargo concordium --help```

    **Note:** To use verifiable builds with cargo-concordium, a container runtime such as Docker is required.

3. Installing Concordium Client

    Concordium Client is a command-line client for interacting with Concordium nodes and sending transactions.

    - Download the tool from the [developer documentation](https://developer.concordium.software/en/mainnet/net/installation/downloads.html#downloads). Once downloaded, the client should be moved to the appropriate directory and its permissions updated:

        ```mv concordium-client_6.1.0 /usr/local/bin/concordium-client```

        ```chmod +x /usr/local/bin/concordium-client```

        **Note:** Concordium client version and location may be different. In the first command, release 6.1.0 is used, and in the second command, the version is removed.

    - To verify Concordium-client installation, use the block-show command to query the newest block via public testnet node by running command:

        ```concordium-client block show --grpc-ip node.testnet.concordium.com```

Now the development environment is ready for smart contract development on Concordium.

## Creating smart contract from a template

A new smart contract project can be created using the Concordium Smart Contracts extension for Visual Studio Code. The steps are as follows:

1. Open the command palette and choose `Initialize a smart contract project`. Then, select the folder for the new project.
2. Choose `default` template (it is sufficient for the current project). Fill in the project title and description.
    - The created project is a regular Cargo Rust project with dependencies listed in `Cargo.toml`, with smart contract in `/src/lib.rs`, and tests in `/tests/tests.rs`.
3. The key difference from regular Rust project is `/deploy-scripts/` folder that has scripts to deploy the smart contract to the blockchain.
4. Open command palette again and choose `Build contract`. It outputs the contract in Wasm format and puts it in `/concordium-out/` folder.
5. To test the smart contract, use command palette `Test contract`.

## Making the "Hello World" dApp

- Modify the Smart Contract: Edit the `lib.rs` file to create a simple smart contract that initializes with the message "Hello, World!" and provides functionality to update and retrieve the message. Few beginner level smart contract tutorials can be found on [`Concordium Academy portal`](https://academy.concordium.software/beginner-level-tutorials).
- Build and Test the Smart Contract: Once the contract is modified, use the `Build Contract` and `Test Contract` options in VS Code to compile and test the contract.

For developers interested in building a frontend for the dApp, advanced tutorials covering frontend integration can be found in: [examples of other dApps](https://developer.concordium.software/en/mainnet/net/guides/dapp-examples.html#dapp-examples).

Now "Hello world" dApp on Concordium is ready!

Features of the "Hello world" dApp:

### Functionality
- **Initialization**: When deployed, the smart contract initializes with the message: "Hello, World!"
- **Update Message**: The contract provides a method to update the stored message
- **Get Message**: It allows querying the current message

### Methods
- `contract_init`: Initializes the contract with "Hello, World!" message
- `update_message`: Updates the message stored in the contract's state to "Hello again, World!"
- `get_message`: Retrieves the current message from the contract's state

### Developer Notes
- This contract is intended for educational purposes and to demonstrate the capabilities of smart contract
  development on Concordium.
- Error handling is simplified in this example. More robust error management is necessary for production.

# Potential improvements 

<!-- write down any friction or improvements you think may be made to the developer experience in a separate note. -->

- "Hello world" dApp is a basic smart contract dApp, it can be extended to include interactive features or to serve as a template for complex applications that require state management and user interaction through transactions.

- In order to implement integrations to Concordium Wallet for Web or Mobile, use NPM library [`@concordium/react-components`](https://www.npmjs.com/package/@concordium/react-components).

- A potential use case for "Hello world" dApp is retrieving 'Hello World' text field from the blockchain and/or sending it to another address after user authentication.

- Functionality for upgrading smart contract in the "Hello world" dApp could be added. It is a good practice to design smart contracts allowing future upgrades. This approach saves time and helps to maintain the continuity and integrity of the smart contract over its lifecycle.