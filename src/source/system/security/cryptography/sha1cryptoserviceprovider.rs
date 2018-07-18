use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xa16537bc, 0x1edf, 0x3516, 0xb7, 0x5e, 0xcc, 0x65, 0xca, 0xf8, 0x73, 0xab)]
interface _SHA1CryptoServiceProvider(_SHA1CryptoServiceProviderVtbl): IDispatch(IDispatchVtbl)  
{}} //SHA1