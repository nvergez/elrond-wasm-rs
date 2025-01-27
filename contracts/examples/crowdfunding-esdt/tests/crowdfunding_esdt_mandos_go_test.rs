#[test]
fn crowdfunding_claim_failed_go() {
    mx_sc_debug::scenario_go("scenarios/crowdfunding-claim-failed.scen.json");
}

#[test]
fn crowdfunding_claim_successful_go() {
    mx_sc_debug::scenario_go("scenarios/crowdfunding-claim-successful.scen.json");
}

#[test]
fn crowdfunding_claim_too_early_go() {
    mx_sc_debug::scenario_go("scenarios/crowdfunding-claim-too-early.scen.json");
}

#[test]
fn crowdfunding_fund_go() {
    mx_sc_debug::scenario_go("scenarios/crowdfunding-fund.scen.json");
}

#[test]
fn crowdfunding_fund_too_late_go() {
    mx_sc_debug::scenario_go("scenarios/crowdfunding-fund-too-late.scen.json");
}

#[test]
fn crowdfunding_init_go() {
    mx_sc_debug::scenario_go("scenarios/crowdfunding-init.scen.json");
}

#[test]
fn egld_crowdfunding_claim_failed_go() {
    mx_sc_debug::scenario_go("scenarios/egld-crowdfunding-claim-failed.scen.json");
}

#[test]
fn egld_crowdfunding_claim_successful_go() {
    mx_sc_debug::scenario_go("scenarios/egld-crowdfunding-claim-successful.scen.json");
}

#[test]
fn egld_crowdfunding_claim_too_early_go() {
    mx_sc_debug::scenario_go("scenarios/egld-crowdfunding-claim-too-early.scen.json");
}

#[test]
fn egld_crowdfunding_fund_go() {
    mx_sc_debug::scenario_go("scenarios/egld-crowdfunding-fund.scen.json");
}

#[test]
fn egld_crowdfunding_fund_too_late_go() {
    mx_sc_debug::scenario_go("scenarios/egld-crowdfunding-fund-too-late.scen.json");
}

#[test]
fn egld_crowdfunding_init_go() {
    mx_sc_debug::scenario_go("scenarios/egld-crowdfunding-init.scen.json");
}
