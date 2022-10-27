const HistoryVerification = require('./pkg');

try{
    const resultString = "AAh/CAAAAAEGJh/ut6Pw5WiDtPWF5h94fONDaUn+YwXn7Wdt5pFAogAAAAAAAABkAAAAAAAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAA==";
    const resultBuf = Buffer.from(resultString, 'base64');
    console.log(HistoryVerification.hash_transaction_history_result_entry(new Uint8Array(resultBuf)));
    const transactionString = "AAh/CKBEC6fy2kGDeNx+90PWPbgu2RYwDs0AKqLMJqVtrlrLAAAAAQAAAACaNpLL5YMfjOTdXVEqrAh99LM12sN6He6pHgCRAa1f1QAAAGQACG5PAAAAAwAAAAAAAAADvia3ZzwUSZZxb5ZOnGVRlscuJckAAAAAAAAAAAAAAAAAAAABAAAAAQAAAAD9anuOI/ouLiE/mq2w+EA0AbfK8hHiXe2tI7JEN58A3gAAAAAAAAAAH8I0q455HrnoWzB2A3/sEMGV82k+R/nl19aSArfm6fcAAAp87inUggAAAAAAAAABAa1f1QAAAEAzoLFNOGlepvZMN/HEDs/fp8KwM2w5OaawhwTXvIRzcQBJZts/auvsyHBoVWYzpXNBWujhzGxTj7Z6H/5HBEoKAAAAAA==";
    const transactionBuf= Buffer.from(transactionString, 'base64');
    console.log(HistoryVerification.hash_transaction_history_entry(new Uint8Array(transactionBuf)));
} catch (e){
    console.log('error')
    console.log(e)
}