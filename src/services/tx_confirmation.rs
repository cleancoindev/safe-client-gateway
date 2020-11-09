use crate::config::base_transaction_service_url;
use crate::providers::ethereum::types::Bytes;
use crate::providers::ethereum::{to_string_result, Call, CallOptions, EthereumProvider};
use crate::utils::cache::Cache;
use crate::utils::context::Context;
use crate::utils::errors::{ApiError, ApiResult};
use ethabi;
use ethabi_contract::use_contract;
use ethereum_types::{Address, U256};
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

pub fn request_nonce_and_data(safe_address: String, context: &Context) -> ApiResult<U256> {
    let eth_provider = EthereumProvider::new(context);

    log::info!("SAFE: {:#?}", &safe_address);
    let address = serde_json::from_value(serde_json::value::Value::String(safe_address))?;
    let call = Call {
        to: Some(address),
        value: None,
        data: Some(safe::functions::nonce::encode_input().into()),
        gas: None,
        gas_price: None,
        from: None,
    };
    let options = CallOptions {
        block: "latest".to_string(),
    };

    log::info!("CALL: {:#?}", &call);

    let simulate_result = eth_provider.call(&call, &options)?;
    let bytes: Bytes = to_string_result(simulate_result)?.into();
    let success = safe::functions::nonce::decode_output(&bytes.0)?;

    Ok(U256::from(success))
}
