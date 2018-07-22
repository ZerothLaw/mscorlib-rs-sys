use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x3b274703, 0xdfae, 0x3f9c, 0xa1, 0xb5, 0x99, 0x90, 0xdf, 0x9d, 0x7f, 0xa3)]
interface _SHA256(_SHA256Vtbl): IDispatch(IDispatchVtbl)  
{}} //HashAlgorithm