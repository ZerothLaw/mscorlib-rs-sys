use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x9cf4339a, 0x2911, 0x3b8a, 0x8f, 0x30, 0xe5, 0xc6, 0xb5, 0xbe, 0x9a, 0x29)]
interface _StackOverflowException(_StackOverflowExceptionVtbl): IDispatch(IDispatchVtbl)  
{}} //SystemException