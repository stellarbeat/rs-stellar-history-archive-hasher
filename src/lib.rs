use stellar_xdr::{Error, ReadXdr, TransactionHistoryResultEntry, WriteXdr};
use sha2::{Digest, Sha256};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct HashResult<const L: usize> {
    pub ledger: u32,
    pub hash: [u8;L],
}

pub fn hash_transaction_history_result_entry<'a>(bytes: impl AsRef<[u8]>) -> Result<HashResult<32>, Error> {
    let transaction_history_result_entry = TransactionHistoryResultEntry::from_xdr(bytes)?;
    let xdr = transaction_history_result_entry.tx_result_set.to_xdr()?;
    let hash = Sha256::digest(&xdr).into();

    return Ok(HashResult {
        hash,
        ledger: transaction_history_result_entry.ledger_seq,
    });
}

