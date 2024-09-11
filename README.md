# 'Hello World' dApp Tutorial

## Introduction

This tutorial is intended for blockchain developers to show how to install Concordium and write the first "Hello World" dApp.

## Why Concordium

For blockchain developers exploring new platforms, Concordium offers a range of appealing features. 

- At its core, Concordium is a leader in blockchain innovation, consistently enhancing its technology. With a dedicated `research center` in Denmark, the focus is on improving security and efficiency through technologies like Zero-Knowledge proofs and new consensus mechanisms. These innovations are regularly integrated into the blockchain. 

- Secondly, Concordium also stands out for its `commitment to environmental` sustainability. It is recognized as one of the most energy-efficient blockchain companies, actively supporting projects that mitigate climate change. This commitment is especially important for developers who prioritize environmental impact in their blockchain applications.

- Thirdly, Concordium specializes in applications that require reliable `identification and secure` financial transactions. The platform is designed to enable the development of apps that prioritize user privacy and functionality. 

- Lastly, becoming part of the Concordium community means access to a `growing community and support`, including financial incentives through grants. These grants are awarded based on decentralized voting by the foundation's board members, ensuring a democratic and equitable distribution of resources.

For developers transitioning from other networks, Concordium offers specific resources to ease the transition and answer common questions. There are  FAQs available for developers coming from Ethereum and Solana, providing guidance on how to start using Concordium effectively. For Ethereum developers, find out more [here](https://developer.concordium.software/en/mainnet/smart-contracts/onboarding-guide-ethereum-developers/faq.html). For those from Solana, access your guide [here](https://developer.concordium.software/en/mainnet/smart-contracts/onboarding-guide-solana-developers/faq.html).

## Important to know

<!-- how you can explain a complex project to a developer who is new to Concordium, expecting them to know what blockchain is.  -->

## Setting up the environment

The set of tools is needed to set the environment correctly.

### The tools

1. Rust
    - If you do not have Rust for Unix, Linux or MacOS, open the terminal and use the command 

        ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```

        Get the latest version using command

        ```rustup update```

        If you are using Windows, download and run `rustup-init.exe` and then follow onscreen instructions.

    - After the installation, you will have 3 tools:
        1. `rustc` - the Rust compiler
        2. `rustup` - the tool for updating your Rust tools, including the Rust compiler
        3. `cargo` - a package manager for Rust. It is the one you will use most frequently.

    - The on-chain language for Concordium smart contracts is Web Assembly (Wasm). Therefore you also need to install the Wasm target, which is used to compile Rust contracts to Wasm: 

        ```rustup target add wasm32-unknown-unknown```

2. Cargo Concordium

    `cargo-concordium` is a tool for developing smart contracts for the Concordium blockchain. It can be used for compiling and testing smart contracts, and enables features such as building contract schemas.

	```cargo install --locked cargo-concordium```

    To see all the subcommands available, run:

    ```cargo concordium --help```

    **Note:** To use verifiable builds with cargo-concordium a container runtime such as Docker is required.

3. Concordium Client

    Concordium Client is a command line client for interacting with the concordium nodes, including sending the transactions.

    - Download the tool from the developer documentation [here](https://developer.concordium.software/en/mainnet/net/installation/downloads.html#downloads)

        ```mv concordium-client_6.1.0 /usr/local/bin/concordium-client```

        ```chmod +x /usr/local/bin/concordium-client```

        **Note:** The location and the version of concordium client may differ for you. In the first command, release 6.1.0 is used, and in the second command, the version is removed.

    - With concordium-client installed, you can use the block-show command to query the newest block via public testnet node:

        ```concordium-client block show --grpc-ip node.testnet.concordium.com```

        After that, you are ready to start developing Concordium smart contracts.

## Developing a smart contract

- The smart contracts can be generated from a template, using VS Code extension for Concordium Smart Contracts: `Concordium Smart Contracts`.
- Open the command palette (or by pressing `Cmd + Shift + P` on MacOS or `Ctrl + Shift + P` on Windows) and choose `Concordium smart contracts: Initialize a smart contract project`. Then, pick up the folder for the project.
- Then choose `default` template (this is enough for the first project). Give it a name and provide a description.
- The project created is a regular Cargo Rust project with its dependencies in `Cargo.toml` file and a smart contract `/src/` folder in `lib.rs` file, and with tests in `/tests/tests.rs` file.
- A difference with typical Rust projects is that `/deploy-scripts/` folder has scripts to deploy the smart contract into the chain.
- Open the command palette again and choose `Concordium smart contracts: Build contract`. It outputs the contract in wasm format and puts in `/concordium-out/` folder.
- To test the smart contract, use the command palette `Concordium smart contracts: Test contract`.

## Making dApp "Hello World"

- Modify `lib.rs` file from template to make it "Hello World" dApp. This example demonstrates how "Hello world" can be implemented with smart
contracts on Concordium.
- Build and then test the smart contract.

The example dApp has its features:

### Functionality
- **Initialization**: When deployed, the smart contract initializes with a default message, "Hello, World!".
- **Update Message**: Functionality to update the stored message.
- **Get Message**: It allows querying the current message.

### Methods
- `contract_init`: Initializes the contract with the default "Hello, World!" message.
- `update_message`: Updates the message stored in the contract's state to "Hello again, World!".
- `get_message`: Retrieves the current message from the contract's state.

### Usage
- The `contract_init` method is called automatically upon deployment, setting the initial message.
- The `update_message` method can be called by users to change the message stored in the contract.
- The `get_message` method can be used to view the current message without altering the state.

### Developer Notes
- This contract is intended for educational purposes and to demonstrate the capabilities of smart contract
  development on Concordium.
- Error handling is simplified in this example. More robust error management would be necessary for production use.

# Additional note

## Potential improvements 

<!-- write down any friction or improvements you think may be made to the developer experience in a separate note. -->

- The dapp example used here is a basic smart contract, it could be extended to include more interactive features or to serve as a template for more complex applications that require state management and user interaction through transactions.

- A potential use case for this "Hello world" dApp could be the following: retrieving the 'Hello World' text field from the blockchain and/or sending it to another address after authentication.

- Not all smart contracts are designed to be upgradeable, which underscores the importance of considering potential future enhancements during their initial development. Planning for upgrades from the outset allows developers to adapt and expand features without the need to rewrite the entire contract each time complexities increase. This approach not only saves time but also helps in maintaining the continuity and integrity of the smart contract over its lifecycle.

## Disclaimers

- There is a possibility to make an interaction frontend part for dapp in order to demonstrate how the smart contract works. It is not covered in the current tutorial, but other [examples of other dapps](https://developer.concordium.software/en/mainnet/net/guides/dapp-examples.html#dapp-examples) have those UIs. 

- In order to implement integrations to Concordium Wallet for Web or Mobile, NPM library [`@concordium/react-components`](https://www.npmjs.com/package/@concordium/react-components) can be used.