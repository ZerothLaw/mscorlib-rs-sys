use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x9ff67f8e, 0xa7aa, 0x3ba6, 0x90, 0xee, 0x9d, 0x44, 0xaf, 0x6e, 0x2f, 0x8c)]
interface _RSAPKCS1KeyExchangeFormatter(_RSAPKCS1KeyExchangeFormatterVtbl): IDispatch(IDispatchVtbl)  
{}} //AsymmetricKeyExchangeFormatter