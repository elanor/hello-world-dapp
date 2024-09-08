# 'Hello World' dApp Tutorial

## Important to know

For Ethereum or Solana developers, the onboarding may be different. Follow these FAQs to answer most common questions for [Ethereum](https://developer.concordium.software/en/mainnet/smart-contracts/onboarding-guide-ethereum-developers/faq.html) and [Solana](https://developer.concordium.software/en/mainnet/smart-contracts/onboarding-guide-solana-developers/faq.html) developers

## Setting up the environment

### The tools needed:

1. Rust
    If you do not have Rust, install it using
    `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    Get the latest version using command
    `rustup update`
    When you install Rustup you’ll also get the latest stable version of the Rust build tool and package manager, also known as Cargo.

    Install the extension to your IDE (for VS Code users: [Rust-analyser](https://code.visualstudio.com/docs/languages/rust) )
2. Cargo Concordium

	`cargo install cargo-concordium`



3. Installing Concordium Client

    Download the dev documentation at https://developer.concordium.software/en/mainnet/net/installation/downloads.html

    `mv concordium-client_6.1.0 /usr/local/bin/concordium-client`

    `chmod +x /usr/local/bin/concordium-client`