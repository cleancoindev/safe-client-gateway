use crate::models::commons::Operation;
use crate::providers::ethereum::transaction::Transaction;
use crate::providers::ethereum::types::Bytes;
use ethereum_types::{Address, H160, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SafeTransaction {
    pub to: Address,
    pub value: U256,
    pub data: Bytes,
    pub nonce: U256,
    pub operation: U256,
    pub safe_tx_gas: U256,
    pub base_gas: U256,
    pub gas_price: U256,
    pub gas_token: Address,
    pub refund_receiver: Address,
}

impl SafeTransaction {
    pub fn new(to: Address, value: U256, data: Bytes, nonce: U256) -> Self {
        Self {
            to,
            value,
            data,
            nonce,
            operation: U256::zero(),
            safe_tx_gas: U256::zero(),
            base_gas: U256::zero(),
            gas_price: U256::zero(),
            gas_token: Address::zero(),
            refund_receiver: Address::zero(),
        }
    }

    pub fn to_ethereum_tx(&self) -> Transaction {
        Transaction {
            nonce: self.nonce,
            gas_price: self.gas_price,
            gas: Default::default(),
            to: Some(self.to),
            value: self.value,
            data: &self.data,
        }
    }
}
