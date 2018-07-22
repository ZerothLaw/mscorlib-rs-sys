use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xa9c9f3d9, 0xe153, 0x39b8, 0xa5, 0x33, 0xb8, 0xdf, 0x46, 0x64, 0x40, 0x7b)]
interface _PolicyException(_PolicyExceptionVtbl): IDispatch(IDispatchVtbl)  
{}} //System.Exception