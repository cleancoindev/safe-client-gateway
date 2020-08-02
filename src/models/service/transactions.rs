use serde::Serialize;
use crate::models::commons::DataDecoded;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub id: String,
    pub timestamp: i64,
    pub tx_status: TransactionStatus,
    pub tx_info: TransactionInfo,
    pub execution_info: Option<ExecutionInfo>,
}

#[derive(Serialize, Debug)]
pub enum TransactionStatus {
    AwaitingConfirmations,
    AwaitingExecution,
    Cancelled,
    Failed,
    Success,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum TransactionInfo {
    Transfer(Transfer),
    SettingsChange(SettingsChange),
    Custom(Custom),
    Unknown,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionInfo {
    pub nonce: u64,
    pub confirmations_required: u64,
    pub confirmations_submitted: u64,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    pub sender: String,
    pub recipient: String,
    pub transfer_info: TransferInfo,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferInfo {
    Erc20(Erc20Transfer),
    Erc721(Erc721Transfer),
    Ether(EtherTransfer),
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Erc20Transfer {
    pub token_address: String,
    pub token_name: Option<String>,
    pub token_symbol: Option<String>,
    pub logo_uri: Option<String>,
    pub decimals: Option<u64>,
    pub value: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Erc721Transfer {
    pub token_address: String,
    pub token_id: String,
    pub token_name: Option<String>,
    pub token_symbol: Option<String>,
    pub logo_uri: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EtherTransfer {
    pub value: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SettingsChange {
    pub data_decoded: DataDecoded
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Custom {
    pub to: String,
    pub data_size: String,
    pub value: String,
}
