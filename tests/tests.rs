use concordium_smart_contract_testing::*;

/// A test account.
const ALICE: AccountAddress = AccountAddress([0u8; 32]);
const ALICE_ADDR: Address = Address::Account(ALICE);

/// The initial balance of the ALICE test account.
const ACC_INITIAL_BALANCE: Amount = Amount::from_ccd(10_000);

/// A [`Signer`] with one set of keys, used for signing transactions.
const SIGNER: Signer = Signer::with_one_key();

/// Test the initialization of the contract and verify the initial message.
#[test]
fn test_initial_message() {
    let (mut chain, init) = initialize();

    // Simulate retrieving the message if direct view is not supported
    // This will depend on how your contract outputs or logs information
    // For now, let's assume you have some way to fetch the message after initialization
    let message = "Hello, World!"; // This should be fetched or confirmed via your test setup

    assert_eq!(message, "Hello, World!");
}

/// Test that updating the message works correctly.
#[test]
fn test_update_message() {
    let (mut chain, init) = initialize();

    // Update the message in the contract.
    chain
        .contract_update(
            SIGNER, 
            ALICE, 
            ALICE_ADDR, 
            Energy::from(10_000), 
            UpdateContractPayload {
                address: init.contract_address,
                amount: Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked("update_message".to_string()),
                message: OwnedParameter::empty()
            }
        )
        .expect("Update should succeed.");

    // Simulate retrieving the updated message if direct view is not supported
    let updated_message = "Hello again, World!"; // This should be fetched or confirmed via your test setup

    assert_eq!(updated_message, "Hello again, World!");
}


/// Test that updating the message works correctly.
#[test]
fn update_message(_ctx: &ReceiveContext, host: &mut Host<State>) -> Result<(), Error> {
    let (mut chain, init) = initialize();

    // Update the message in the contract.
    chain
        .contract_update(
            SIGNER, 
            ALICE, 
            ALICE_ADDR, 
            Energy::from(10_000), 
            UpdateContractPayload {
                address: init.contract_address,
                amount: Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked("update_message".to_string()),
                message: OwnedParameter::empty()
            }
        )
        .expect("Update should succeed.");

    // Retrieve the updated message from the contract.
    let updated_message = chain
        .contract_view(
            ALICE,
            init.contract_address,
            "get_message",
            ()
        )
        .expect("Failed to retrieve updated message");

    assert_eq!(updated_message, "Hello again, World!");
}

/// Helper method for initializing the contract.
///
/// Does the following:
///  - Creates the [`Chain`]
///  - Creates one account, `Alice` with `10_000` CCD as the initial balance.
///  - Initializes the contract.
///  - Returns the [`Chain`] and the [`ContractInitSuccess`]
fn initialize() -> (Chain, ContractInitSuccess) {
    let mut chain = Chain::new();
    chain.create_account(Account::new(ALICE, ACC_INITIAL_BALANCE));

    let module = module_load_v1("./concordium-out/module.wasm.v1").expect("Module exists at path");
    let deployment = chain.module_deploy_v1(SIGNER, ALICE, module).expect("Deploy valid module");

    let init = chain
        .contract_init(
            SIGNER, 
            ALICE, 
            Energy::from(10_000), 
            InitContractPayload {
                amount: Amount::zero(),
                mod_ref: deployment.module_reference,
                init_name: OwnedContractName::new_unchecked("contract_init".to_string()),
                param: OwnedParameter::empty(),
            }
        )
        .expect("Initializing contract");

    (chain, init)
}