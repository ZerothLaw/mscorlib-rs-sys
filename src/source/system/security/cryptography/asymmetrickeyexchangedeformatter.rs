use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xb6685cca, 0x7a49, 0x37d1, 0xa8, 0x05, 0x3d, 0xe8, 0x29, 0xcb, 0x8d, 0xeb)]
interface _AsymmetricKeyExchangeDeformatter(_AsymmetricKeyExchangeDeformatterVtbl): IDispatch(IDispatchVtbl)  
{}} //abstract