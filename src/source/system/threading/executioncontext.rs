use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x56d201f1, 0x3e5d, 0x39d9, 0xb5, 0xde, 0x06, 0x47, 0x10, 0x81, 0x89, 0x05)]
interface _ContextCallback(_ContextCallbackVtbl): IDispatch(IDispatchVtbl)  
{}} //delegate