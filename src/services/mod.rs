use crate::models::commons::PageMetadata;
use std::cmp::max;

pub mod about;
pub mod balances;
pub mod hooks;
pub mod transactions_builder;
pub mod transactions_details;
pub mod transactions_history;
pub mod transactions_list;
pub mod transactions_queued;
pub mod tx_confirmation;

#[cfg(test)]
mod tests;

pub fn offset_page_meta(meta: &PageMetadata, offset: i64) -> String {
    PageMetadata {
        offset: (max(0, (meta.offset as i64) + offset)) as u64,
        limit: meta.limit,
    }
    .to_url_string()
}
