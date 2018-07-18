use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xe5456726, 0x33f6, 0x34e4, 0x95, 0xc2, 0xdb, 0x2b, 0xfa, 0x58, 0x14, 0x62)]
interface _HMAC(_HMACVtbl): IDispatch(IDispatchVtbl)  
{}} //KeyedHashAlgorithm