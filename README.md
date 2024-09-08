# 'Hello World' dApp Tutorial

## Introduction

<!-- write a short tutorial on how to install Concordium and write the first 'Hello World' dApp -->

## Why Concordium

<!-- mention why developers need to use Concordium -->

## Important to know

<!-- how you can explain a complex project to a developer who is new to Concordium, expecting them to know what blockchain is.  -->

For Ethereum or Solana developers, the onboarding may be different. Follow these FAQs to answer most common questions for [Ethereum](https://developer.concordium.software/en/mainnet/smart-contracts/onboarding-guide-ethereum-developers/faq.html) and [Solana](https://developer.concordium.software/en/mainnet/smart-contracts/onboarding-guide-solana-developers/faq.html) developers

## Setting up the environment

### The tools needed:

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

    - Download the tool from the the developer documentation at https://developer.concordium.software/en/mainnet/net/installation/downloads.html

    ```mv concordium-client_6.1.0 /usr/local/bin/concordium-client```

    ```chmod +x /usr/local/bin/concordium-client```

    ## Info

    - The smart contracts can be generated from a template, using VS Code extension for Concordium Smart Contracts: `Concordium Smart Contracts`.
    - Open the command palette (or by pressing `Cmd + Shift + P` on MacOS or `Ctrl + Shift + P` on Windows) and choose `Concordium smart contracts: Initialize a smart contract project`. Then, pick up the folder for the project.
    - Then choose `default` template (this is enough for the first project). Give it a name and provide a description.
    - The project created is a regular Cargo Rust project with its dependencies in `Cargo.toml` file and a smart contract `/src/` folder in `lib.rs` file, and with tests in `/tests/tests.rs` file.
    - A difference with typical Rust projects is that `/deploy-scripts/` folder has scripts to deploy the smart contract into the chain.
    - Open the command palette again and choose `Concordium smart contracts: Build contract`. It outputs the contract in wasm format and puts in `/concordium-out/` folder.
    - To test the smart contract, we use the command palette `Concordium smart contracts: Test contract`.


    # Additional note

    ## Potential improvements 

    <!-- write down any friction or improvements you think may be made to the developer experience in a separate note. -->

    The dapp example used here is a basic smart contract, it could be extended to include more interactive features or to serve as a template for more complex applications that require state management and user interaction through transactions.

    ## Disclaimers

    - There is a possibility to make an interaction UI for dapp in order to demonstrate how the smart contract works. It is not covered in the current tutorial, but other [examples of other dapps](https://developer.concordium.software/en/mainnet/net/guides/dapp-examples.html#dapp-examples) have those UIs. 

    - In order to implement integrations to Concordium Wallet for Web or Mobile, NPM library [`@concordium/react-components`](https://www.npmjs.com/package/@concordium/react-components) can be used.