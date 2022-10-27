extern crate core;

use std::io;
use stellar_xdr::Error;

#[test]
fn test_happy() {
    let xdr = "AAh/CKBEC6fy2kGDeNx+90PWPbgu2RYwDs0AKqLMJqVtrlrLAAAAAQAAAACaNpLL5YMfjOTdXVEqrAh99LM12sN6He6pHgCRAa1f1QAAAGQACG5PAAAAAwAAAAAAAAADvia3ZzwUSZZxb5ZOnGVRlscuJckAAAAAAAAAAAAAAAAAAAABAAAAAQAAAAD9anuOI/ouLiE/mq2w+EA0AbfK8hHiXe2tI7JEN58A3gAAAAAAAAAAH8I0q455HrnoWzB2A3/sEMGV82k+R/nl19aSArfm6fcAAAp87inUggAAAAAAAAABAa1f1QAAAEAzoLFNOGlepvZMN/HEDs/fp8KwM2w5OaawhwTXvIRzcQBJZts/auvsyHBoVWYzpXNBWujhzGxTj7Z6H/5HBEoKAAAAAA==";
    let bytes = base64::decode(xdr).unwrap();
    let hash = stellar_history_archive_hasher::internal::hash_transaction_history_entry(bytes).unwrap();
    let hash_base64 = base64::encode(hash);
    assert_eq!(hash_base64, "rx/vbqe2TibD5lhk3swAgQzeMud7rPtqZgKdangjWIA=");
}

#[test]
fn test_xdr_not_transaction_history_entry() {
    let xdr = "AAh/CAAAAAEGJh/ut6Pw5WiDtPWF5h94fONDaUn+YwXn7Wdt5pFAogAAAAAAAABkAAAAAAAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAA==";
    let bytes = base64::decode(xdr).unwrap();
    let result = stellar_history_archive_hasher::internal::hash_transaction_history_entry(bytes);
    assert_eq!(Err(Error::Io(io::ErrorKind::UnexpectedEof.into())), result)
}