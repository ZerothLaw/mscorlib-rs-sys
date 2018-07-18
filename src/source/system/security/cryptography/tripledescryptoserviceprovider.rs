use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xec69d083, 0x3cd0, 0x3c0c, 0x99, 0x8c, 0x3b, 0x73, 0x8d, 0xb5, 0x35, 0xd5)]
interface _TripleDESCryptoServiceProvider(_TripleDESCryptoServiceProviderVtbl): IDispatch(IDispatchVtbl)  
{}}