use mx_sc_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/digital-cash");

    blockchain.register_contract(
        "file:output/digital-cash.wasm",
        digital_cash::ContractBuilder,
    );
    blockchain
}

// verify_ed25519 not implemented
// #[test]
// fn claim_egld_rs() {
//     mx_sc_debug::scenario_rs("scenarios/claim-egld.scen.json", world());
// }

// verify_ed25519 not implemented
// #[test]
// fn claim_esdt_rs() {
//     mx_sc_debug::scenario_rs("scenarios/claim-esdt.scen.json", world());
// }

#[test]
fn fund_egld_and_esdt_rs() {
    mx_sc_debug::scenario_rs("scenarios/fund-egld-and-esdt.scen.json", world());
}

#[test]
fn set_accounts_rs() {
    mx_sc_debug::scenario_rs("scenarios/set-accounts.scen.json", world());
}

#[test]
fn withdraw_egld_rs() {
    mx_sc_debug::scenario_rs("scenarios/withdraw-egld.scen.json", world());
}

#[test]
fn withdraw_esdt_rs() {
    mx_sc_debug::scenario_rs("scenarios/withdraw-esdt.scen.json", world());
}
