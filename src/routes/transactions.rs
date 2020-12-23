use crate::config::request_cache_duration;
use crate::models::requests::transfer_request::TransferRequest;
use crate::models::service::transactions::requests::ConfirmationRequest;
use crate::services::{
    transactions_builder, transactions_details, transactions_history, transactions_list,
    transactions_queued, tx_confirmation,
};
use crate::utils::cache::CacheExt;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use ethereum_types::U256;
use rocket::http::ext::IntoCollection;
use rocket::response::content;
use rocket_contrib::json::Json;

#[get("/v1/safes/<safe_address>/transactions?<page_url>")]
pub fn all(
    context: Context,
    safe_address: String,
    page_url: Option<String>,
) -> ApiResult<content::Json<String>> {
    context
        .cache()
        .cache_resp(&context.uri(), request_cache_duration(), || {
            transactions_list::get_all_transactions(&context, &safe_address, &page_url)
        })
}

#[get("/v1/transactions/<details_id>")]
pub fn details(context: Context, details_id: String) -> ApiResult<content::Json<String>> {
    context
        .cache()
        .cache_resp(&context.uri(), request_cache_duration(), || {
            transactions_details::get_transactions_details(&context, &details_id)
        })
}

#[post(
    "/v1/transactions/<safe_tx_hash>/confirmations",
    data = "<tx_confirmation_request>"
)]
pub fn submit_confirmation(
    context: Context,
    safe_tx_hash: String,
    tx_confirmation_request: Json<ConfirmationRequest>,
) -> ApiResult<content::Json<String>> {
    tx_confirmation::submit_confirmation(
        &context,
        &safe_tx_hash,
        &tx_confirmation_request.signed_safe_tx_hash,
    )
    .and_then(|_| {
        context
            .cache()
            .cache_resp(&context.uri(), request_cache_duration(), || {
                transactions_details::get_transactions_details(&context, &safe_tx_hash)
            })
    })
}

#[get("/v1/safes/<safe_address>/transactions/history?<page_url>&<timezone_offset>")]
pub fn history_transactions(
    context: Context,
    safe_address: String,
    page_url: Option<String>,
    timezone_offset: Option<String>,
) -> ApiResult<content::Json<String>> {
    context
        .cache()
        .cache_resp(&context.uri(), request_cache_duration(), || {
            transactions_history::get_history_transactions(
                &context,
                &safe_address,
                &page_url,
                &timezone_offset,
            )
        })
}

#[get("/v1/safes/<safe_address>/transactions/queued?<page_url>&<timezone_offset>&<trusted>")]
pub fn queued_transactions(
    context: Context,
    safe_address: String,
    page_url: Option<String>,
    timezone_offset: Option<String>,
    trusted: Option<bool>,
) -> ApiResult<content::Json<String>> {
    context
        .cache()
        .cache_resp(&context.uri(), request_cache_duration(), || {
            transactions_queued::get_queued_transactions(
                &context,
                &safe_address,
                &page_url,
                &timezone_offset,
                &trusted,
            )
        })
}

#[post("/v1/safes/<safe_address>/data", data = "<transfer_request>")]
pub fn request_tx_hash_for_transfer(
    context: Context,
    safe_address: String,
    transfer_request: Json<TransferRequest>,
) -> ApiResult<()> {
    // ApiResult<content::Json<String>> {
    transactions_builder::request_nonce_and_data(
        &context,
        safe_address,
        transfer_request.into_inner(),
    );
    // .map(|it| content::Json(serde_json::to_string(it.as_ref()).unwrap()))
    Ok(())
}
