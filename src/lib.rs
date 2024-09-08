//! An example demonstrating how "Hello world" can be implemented with smart
//! contracts on Concordium.
//! 
//! It is designed to show the basics of contract setup, state management, and simple interaction patterns in the
//! context of a decentralized application (dApp).
//!
//! ## Functionality
//! - **Initialization**: When deployed, the smart contract initializes with a default message, "Hello, World!".
//! - **Update Message**: Functionality to update the stored message.
//! - **Get Message**: It allows querying the current message.
//!
//! ## Methods
//! - `contract_init`: Initializes the contract with the default "Hello, World!" message.
//! - `update_message`: Updates the message stored in the contract's state to "Hello again, World!".
//! - `get_message`: Retrieves the current message from the contract's state.
//!
//! ## Usage
//! - The `contract_init` method is called automatically upon deployment, setting the initial message.
//! - The `update_message` method can be called by users to change the message stored in the contract.
//! - The `get_message` method can be used to view the current message without altering the state.
//!
//! ## Example
//! This contract could be extended to include more interactive features or to serve as a template for more complex
//! applications that require state management and user interaction through transactions.
//!
//! ## Developer Notes
//! - This contract is intended for educational purposes and to demonstrate the capabilities of smart contract
//!   development on Concordium.
//! - Error handling is simplified in this example. More robust error management would be necessary for production use.

#![cfg_attr(not(feature = "std"), no_std)]

use concordium_std::*;

/// The state of the smart contract, which in this case will hold our Hello World message.
#[derive(Serialize, SchemaType)]
pub struct State {
    message: String,
}

/// Errors that could be returned by the smart contract functions.
#[derive(Debug, PartialEq, Eq, Reject, Serialize, SchemaType)]
pub enum Error {
    /// Error emitted if any operation fails (placeholder for more specific errors)
    OperationFailed,
}

#[init(contract = "hello_world", parameter = "()")]
fn contract_init(_ctx: &InitContext, _state_builder: &mut StateBuilder) -> InitResult<State> {
    Ok(State {
        message: "Hello, World!".to_string(),
    })
}

#[receive(
    contract = "hello_world",
    name = "update_message",
    parameter = "()",
    mutable
)]
fn update_message(_ctx: &ReceiveContext, host: &mut Host<State>) -> Result<(), Error> {
    host.state_mut().message = "Hello again, World!".to_string();
    Ok(())
}

#[receive(contract = "hello_world", name = "get_message", return_value = "String")]
fn get_message<'a>(_ctx: &ReceiveContext, host: &'a Host<State>) -> ReceiveResult<&'a State> {
    Ok(host.state())
}
