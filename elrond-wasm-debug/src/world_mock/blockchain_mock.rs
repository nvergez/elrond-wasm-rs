use alloc::{boxed::Box, vec::Vec};
use elrond_wasm::types::Address;
use num_bigint::BigUint;
use num_traits::Zero;
use std::collections::HashMap;

use crate::{
    esdt_transfer_event_log,
    tx_mock::{SendBalance, TxContext, TxInput, TxLog, TxOutput, TxPanic},
    world_mock::AccountEsdt,
    ContractMap,
};

use super::{AccountData, BlockInfo, BlockchainMockError};

const ELROND_REWARD_KEY: &[u8] = b"ELRONDreward";

pub struct BlockchainMock {
    pub accounts: HashMap<Address, AccountData>,
    pub new_addresses: HashMap<(Address, u64), Address>,
    pub previous_block_info: BlockInfo,
    pub current_block_info: BlockInfo,
}

impl BlockchainMock {
    pub fn new() -> Self {
        BlockchainMock {
            accounts: HashMap::new(),
            new_addresses: HashMap::new(),
            previous_block_info: BlockInfo::new(),
            current_block_info: BlockInfo::new(),
        }
    }
}

impl Default for BlockchainMock {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockchainMock {
    pub fn send_balance(
        &mut self,
        contract_address: &Address,
        send_balance_list: &[SendBalance],
        result_logs: &mut Vec<TxLog>,
    ) -> Result<(), BlockchainMockError> {
        for send_balance in send_balance_list {
            if send_balance.token_identifier.is_empty() {
                self.subtract_egld_balance(contract_address, &send_balance.amount)?;
                self.increase_egld_balance(&send_balance.recipient, &send_balance.amount);
            } else {
                let esdt_token_identifier = send_balance.token_identifier.as_slice();
                let esdt_nonce = send_balance.nonce;
                self.subtract_esdt_balance(
                    contract_address,
                    esdt_token_identifier,
                    esdt_nonce,
                    &send_balance.amount,
                );
                self.increase_esdt_balance(
                    &send_balance.recipient,
                    esdt_token_identifier,
                    esdt_nonce,
                    &send_balance.amount,
                );

                let log = esdt_transfer_event_log(
                    contract_address.clone(),
                    send_balance.recipient.clone(),
                    esdt_token_identifier.to_vec(),
                    &send_balance.amount,
                );
                result_logs.insert(0, log); // TODO: it's a hack, should be inserted during execution, not here
            }
        }
        Ok(())
    }

    pub fn increase_nonce(&mut self, address: &Address) {
        let account = self.accounts.get_mut(address).unwrap_or_else(|| {
            panic!(
                "Account not found: {}",
                &std::str::from_utf8(address.as_ref()).unwrap()
            )
        });
        account.nonce += 1;
    }

    pub fn create_account_after_deploy(
        &mut self,
        tx_input: &TxInput,
        new_storage: HashMap<Vec<u8>, Vec<u8>>,
        contract_path: Vec<u8>,
    ) -> Address {
        let sender = self
            .accounts
            .get(&tx_input.from)
            .unwrap_or_else(|| panic!("Unknown deployer"));
        let sender_nonce_before_tx = sender.nonce - 1;
        let new_address = self
            .get_new_address(tx_input.from.clone(), sender_nonce_before_tx)
            .unwrap_or_else(|| {
                panic!("Missing new address. Only explicit new deploy addresses supported")
            });

        let old_value = self.accounts.insert(
            new_address.clone(),
            AccountData {
                address: new_address.clone(),
                nonce: 0,
                egld_balance: tx_input.egld_value.clone(),
                storage: new_storage,
                esdt: AccountEsdt::default(),
                username: Vec::new(),
                contract_path: Some(contract_path),
                contract_owner: Some(tx_input.from.clone()),
            },
        );
        assert!(
            old_value.is_none(),
            "Account already exists at deploy address."
        );

        new_address
    }

    pub fn increase_validator_reward(&mut self, address: &Address, amount: &BigUint) {
        let account = self.accounts.get_mut(address).unwrap_or_else(|| {
            panic!(
                "Account not found: {}",
                &std::str::from_utf8(address.as_ref()).unwrap()
            )
        });
        account.egld_balance += amount;
        let mut storage_v_rew =
            if let Some(old_storage_value) = account.storage.get(ELROND_REWARD_KEY) {
                BigUint::from_bytes_be(old_storage_value)
            } else {
                BigUint::zero()
            };
        storage_v_rew += amount;
        account
            .storage
            .insert(ELROND_REWARD_KEY.to_vec(), storage_v_rew.to_bytes_be());
    }

    pub fn try_set_username(&mut self, address: &Address, username: &[u8]) -> bool {
        let account = self.accounts.get_mut(address).unwrap_or_else(|| {
            panic!(
                "Account not found: {}",
                &std::str::from_utf8(address.as_ref()).unwrap()
            )
        });
        if account.username.is_empty() {
            account.username = username.to_vec();
            true
        } else {
            false
        }
    }
}

pub fn execute_tx(
    tx_context: TxContext,
    contract_identifier: &[u8],
    contract_map: &ContractMap<TxContext>,
) -> TxOutput {
    let func_name = tx_context.tx_input_box.func_name.clone();
    let contract_inst = contract_map.new_contract_instance(contract_identifier, tx_context);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let call_successful = contract_inst.call(func_name.as_slice());
        if !call_successful {
            std::panic::panic_any(TxPanic {
                status: 1,
                message: b"invalid function (not found)".to_vec(),
            });
        }
        let context = contract_inst.into_api();
        context.into_output()
    }));
    match result {
        Ok(tx_output) => tx_output,
        Err(panic_any) => panic_result(panic_any),
    }
}

fn panic_result(panic_any: Box<dyn std::any::Any + std::marker::Send>) -> TxOutput {
    if panic_any.downcast_ref::<TxOutput>().is_some() {
        // async calls panic with the tx output directly
        // it is not a failure, simply a way to kill the execution
        return *panic_any.downcast::<TxOutput>().unwrap();
    }

    if let Some(panic_obj) = panic_any.downcast_ref::<TxPanic>() {
        return TxOutput::from_panic_obj(panic_obj);
    }

    if let Some(panic_string) = panic_any.downcast_ref::<String>() {
        return TxOutput::from_panic_string(panic_string.as_str());
    }

    TxOutput::from_panic_string("unknown panic")
}