use mx_sc_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();

    blockchain.register_contract(
        "file:../kitty-ownership/output/kitty-ownership.wasm",
        kitty_ownership::ContractBuilder,
    );
    blockchain.register_contract(
        "file:output/kitty-auction.wasm",
        kitty_auction::ContractBuilder,
    );

    blockchain
}
#[test]
fn bid_first_rs() {
    mx_sc_debug::scenario_rs("scenarios/bid_first.scen.json", world());
}

#[test]
fn bid_second_max_rs() {
    mx_sc_debug::scenario_rs("scenarios/bid_second_max.scen.json", world());
}

#[test]
fn bid_second_ok_rs() {
    mx_sc_debug::scenario_rs("scenarios/bid_second_ok.scen.json", world());
}

#[test]
fn bid_second_too_low_rs() {
    mx_sc_debug::scenario_rs("scenarios/bid_second_too_low.scen.json", world());
}

#[test]
fn bid_siring_auction_rs() {
    mx_sc_debug::scenario_rs("scenarios/bid_siring_auction.scen.json", world());
}

#[test]
fn create_and_auction_gen_zero_kitty_rs() {
    mx_sc_debug::scenario_rs(
        "scenarios/create_and_auction_gen_zero_kitty.scen.json",
        world(),
    );
}

#[test]
fn create_sale_auction_not_owner_rs() {
    mx_sc_debug::scenario_rs("scenarios/create_sale_auction_not_owner.scen.json", world());
}

#[test]
fn create_sale_auction_ok_rs() {
    mx_sc_debug::scenario_rs("scenarios/create_sale_auction_ok.scen.json", world());
}

#[test]
fn create_siring_auction_not_owner_rs() {
    mx_sc_debug::scenario_rs(
        "scenarios/create_siring_auction_not_owner.scen.json",
        world(),
    );
}

#[test]
fn create_siring_auction_ok_rs() {
    mx_sc_debug::scenario_rs("scenarios/create_siring_auction_ok.scen.json", world());
}

#[test]
fn end_auction_no_bids_rs() {
    mx_sc_debug::scenario_rs("scenarios/end_auction_no_bids.scen.json", world());
}

#[test]
fn end_auction_second_bid_max_early_rs() {
    mx_sc_debug::scenario_rs(
        "scenarios/end_auction_second_bid_max_early.scen.json",
        world(),
    );
}

#[test]
fn end_auction_second_bid_ok_early_rs() {
    mx_sc_debug::scenario_rs(
        "scenarios/end_auction_second_bid_ok_early.scen.json",
        world(),
    );
}

#[test]
fn end_auction_second_bid_ok_late_rs() {
    mx_sc_debug::scenario_rs(
        "scenarios/end_auction_second_bid_ok_late.scen.json",
        world(),
    );
}

#[test]
fn end_siring_auction_rs() {
    mx_sc_debug::scenario_rs("scenarios/end_siring_auction.scen.json", world());
}

#[test]
fn init_rs() {
    mx_sc_debug::scenario_rs("scenarios/init.scen.json", world());
}
