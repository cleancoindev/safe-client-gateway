use ethereum_types::Address;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferRequest {
    pub receiver: Address,
    pub value: u64,
    pub token_address: Option<Address>,
    pub infura_token: String,
}
