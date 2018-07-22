use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x8034aaf4, 0x3666, 0x3b6f, 0x85, 0xcf, 0x46, 0x3f, 0x9b, 0xfd, 0x31, 0xa9)]
interface _RSAPKCS1KeyExchangeDeformatter(_RSAPKCS1KeyExchangeDeformatterVtbl): IDispatch(IDispatchVtbl)  
{}} //AsymmetricKeyExchangeDeformatter