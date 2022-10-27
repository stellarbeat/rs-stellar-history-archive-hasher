const HistoryVerification = require('./pkg');

try{
    const xdrString = "AAh/CAAAAAEGJh/ut6Pw5WiDtPWF5h94fONDaUn+YwXn7Wdt5pFAogAAAAAAAABkAAAAAAAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAA==";
    const buf = Buffer.from(xdrString, 'base64');
    const result = HistoryVerification.hash_transaction_history_result_entry(new Uint8Array(buf));
    console.log(result);
} catch (e){
    console.log('error')
    console.log(e)
}