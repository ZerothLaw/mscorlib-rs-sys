use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x95b525db, 0x6b81, 0x3cdc, 0x8f, 0xe7, 0x71, 0x3f, 0x7f, 0xc7, 0x93, 0xc0)]
interface _ThreadAbortException(_ThreadAbortExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}
