use crate::models::backend::webhooks::{Payload, PayloadDetails};
use crate::utils::cache::Cache;
use anyhow::Result;

pub fn invalidate_caches(cache: &impl Cache, payload: &Payload) -> Result<()> {
    cache.invalidate_pattern(&format!("*{}*", &payload.address));
    payload.details.as_ref().map(|d| match d {
        PayloadDetails::NewConfirmation(data) => {
            cache.invalidate_pattern(&format!("*{}*", data.safe_tx_hash));
        }
        PayloadDetails::ExecutedMultisigTransaction(data) => {
            cache.invalidate_pattern(&format!("*{}*", data.safe_tx_hash));
        }
        PayloadDetails::PendingMultisigTransaction(data) => {
            cache.invalidate_pattern(&format!("*{}*", data.safe_tx_hash));
        }
        _ => {}
    });
    Ok(())
}
