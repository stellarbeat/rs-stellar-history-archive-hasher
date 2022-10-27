use js_sys::{Uint8Array};
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub unsafe fn hash_transaction_history_result_entry(bytes: &[u8]) -> Uint8Array {
    return Uint8Array::view(&internal::hash_transaction_history_result_entry(bytes).unwrap());
}

pub mod internal {
    use stellar_xdr::{Error, ReadXdr, TransactionHistoryResultEntry, WriteXdr};
    use sha2::{Digest, Sha256};

    pub fn hash_transaction_history_result_entry(bytes: impl AsRef<[u8]>) -> Result<[u8; 32], Error> {
        let transaction_history_result_entry = TransactionHistoryResultEntry::from_xdr(bytes)?;
        let xdr = transaction_history_result_entry.tx_result_set.to_xdr()?;

        return Ok(Sha256::digest(&xdr).into());
    }
}