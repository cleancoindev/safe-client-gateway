use crate::config::base_transaction_service_url;
use crate::models::requests::safe_transaction::SafeTransaction;
use crate::models::requests::transfer_request::TransferRequest;
use crate::providers::ethereum::types::Bytes;
use crate::providers::ethereum::{to_string_result, Call, CallOptions, EthereumProvider};
use crate::utils::cache::Cache;
use crate::utils::context::Context;
use crate::utils::errors::{ApiError, ApiResult};
use ethabi;
use ethabi_contract::use_contract;
use ethereum_types::{Address, H256, U256};
use std::collections::HashMap;

use_contract!(safe, "./abis/safe.json");

pub fn submit_confirmation(
    context: &Context,
    safe_tx_hash: &str,
    signature: &str,
) -> ApiResult<()> {
    let url = format!(
        "{}/v1/multisig-transactions/{}/confirmations/",
        base_transaction_service_url(),
        &safe_tx_hash
    );
    let mut json = HashMap::new();
    json.insert("signature", signature);

    let response = context.client().post(&url).json(&json).send()?;

    if response.status().is_success() {
        context
            .cache()
            .invalidate_pattern(&format!("*{}*", &safe_tx_hash));
        Ok(())
    } else {
        Err(ApiError::from_http_response(
            response,
            String::from("Unexpected tx confirmation error"),
        ))
    }
}

pub fn request_nonce_and_data(
    context: &Context,
    safe_address: String,
    transfer_request: TransferRequest,
) -> ApiResult<()> {
    let eth_provider = EthereumProvider::new(context);
    let safe_address: Address =
        serde_json::from_value(serde_json::value::Value::String(safe_address))?;
    let nonce = get_safe_nonce(&eth_provider, &safe_address)?;

    request_hash(&eth_provider, &safe_address, transfer_request, nonce);
    Ok(())
}

fn get_safe_nonce(eth_provider: &EthereumProvider, safe_address: &Address) -> ApiResult<U256> {
    let call = Call {
        to: Some(safe_address.clone()),
        value: None,
        data: Some(safe::functions::nonce::encode_input().into()),
        gas: None,
        gas_price: None,
        from: None,
    };
    let options = CallOptions {
        block: "latest".to_string(),
    };

    let simulate_result = eth_provider.call(&call, &options)?;
    let bytes: Bytes = to_string_result(simulate_result)?.into();
    let success = safe::functions::nonce::decode_output(&bytes.0)?;
    Ok(U256::from(success))
}

fn request_hash(
    eth_provider: &EthereumProvider,
    safe_address: &Address,
    transfer_request: TransferRequest,
    nonce: U256,
) -> ApiResult<()> {
    let safe_transaction = SafeTransaction::new(
        transfer_request.receiver,
        U256::from(transfer_request.value),
        Bytes::from(String::from("0x")),
        nonce,
    );
    // let ethereum_transaction = safe_transaction.to_ethereum_tx();

    let call = Call {
        to: None,
        value: None,
        data: Some(
            safe::functions::get_transaction_hash::encode_input(
                safe_transaction.to,
                safe_transaction.value,
                safe_transaction.data,
                safe_transaction.operation,
                safe_transaction.safe_tx_gas,
                safe_transaction.base_gas,
                safe_transaction.gas_price,
                safe_transaction.gas_token,
                safe_transaction.refund_receiver,
                safe_transaction.nonce,
            )
            .into(),
        ),
        gas: None,
        gas_price: None,
        from: Some(safe_address.clone()),
    };

    let options = CallOptions {
        block: "latest".to_string(),
    };

    let simulate_result = eth_provider.call(&call, &options)?;
    let bytes = to_string_result(simulate_result)?.into_bytes();
    let success: U256 = safe::functions::nonce::decode_output(&bytes)?;

    let success_as_bytes = success.to_string();
    log::info!("result for txHash{:#?}", success_as_bytes);
    Ok(())
}
