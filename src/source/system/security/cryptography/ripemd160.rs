use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xe5481be9, 0x3422, 0x3506, 0xbc, 0x35, 0xb9, 0x6d, 0x45, 0x35, 0x01, 0x4d)]
interface _RIPEMD160(_RIPEMD160Vtbl): IDispatch(IDispatchVtbl)  
{}} //HashAlgorithm