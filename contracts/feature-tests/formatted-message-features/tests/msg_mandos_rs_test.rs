use mx_sc_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/formatted-message-features");

    blockchain.register_contract(
        "file:output/formatted-message-features.wasm",
        formatted_message_features::ContractBuilder,
    );

    blockchain
}

#[test]
fn managed_error_message_rs() {
    mx_sc_debug::scenario_rs("scenarios/managed_error_message.scen.json", world());
}

#[test]
fn sc_format_rs() {
    mx_sc_debug::scenario_rs("scenarios/sc_format.scen.json", world());
}
