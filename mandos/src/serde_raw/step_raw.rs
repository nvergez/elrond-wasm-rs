use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    AccountRaw, BlockInfoRaw, CheckAccountsRaw, NewAddressRaw, TxCallRaw, TxDeployRaw, TxExpectRaw,
    TxQueryRaw, TxTransferRaw, TxValidatorRewardRaw, ValueSubTree,
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "step")]
pub enum StepRaw {
    ExternalSteps {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,

        path: String,
    },

    #[serde(rename_all = "camelCase")]
    SetState {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,

        #[serde(default)]
        #[serde(skip_serializing_if = "BTreeMap::is_empty")]
        accounts: BTreeMap<String, AccountRaw>,

        #[serde(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        new_addresses: Vec<NewAddressRaw>,

        #[serde(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        block_hashes: Vec<ValueSubTree>,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        previous_block_info: Option<BlockInfoRaw>,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        current_block_info: Option<BlockInfoRaw>,
    },

    #[serde(rename_all = "camelCase")]
    ScCall {
        #[serde(default)]
        tx_id: String,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,

        tx: TxCallRaw,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        expect: Option<TxExpectRaw>,
    },

    #[serde(rename_all = "camelCase")]
    ScQuery {
        #[serde(default)]
        tx_id: String,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,

        tx: TxQueryRaw,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        expect: Option<TxExpectRaw>,
    },

    #[serde(rename_all = "camelCase")]
    ScDeploy {
        #[serde(default)]
        tx_id: String,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,

        tx: TxDeployRaw,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        expect: Option<TxExpectRaw>,
    },

    #[serde(rename_all = "camelCase")]
    Transfer {
        #[serde(default)]
        tx_id: String,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,

        tx: TxTransferRaw,
    },

    #[serde(rename_all = "camelCase")]
    ValidatorReward {
        #[serde(default)]
        tx_id: String,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,

        tx: TxValidatorRewardRaw,
    },

    CheckState {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,

        accounts: CheckAccountsRaw,
    },

    DumpState {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
    },
}