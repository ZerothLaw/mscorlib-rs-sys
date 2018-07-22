use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x1365b84b, 0x6477, 0x3c40, 0xbe, 0x6a, 0x08, 0x9d, 0xc0, 0x1e, 0xce, 0xd9)]
interface _AsymmetricKeyExchangeFormatter(_AsymmetricKeyExchangeFormatterVtbl): IDispatch(IDispatchVtbl)  
{}}