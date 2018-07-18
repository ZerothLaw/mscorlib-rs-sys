use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x77a416e7, 0x2ac6, 0x3d0e, 0x98, 0xff, 0x3b, 0xa0, 0xf5, 0x86, 0xf5, 0x6f)]
interface _RSAOAEPKeyExchangeFormatter(_RSAOAEPKeyExchangeFormatterVtbl): IDispatch(IDispatchVtbl)  
{}}