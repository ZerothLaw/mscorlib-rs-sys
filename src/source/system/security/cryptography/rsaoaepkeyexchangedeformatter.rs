use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x37625095, 0x7baa, 0x377d, 0xa0, 0xdc, 0x7f, 0x46, 0x5c, 0x01, 0x67, 0xaa)]
interface _RSAOAEPKeyExchangeDeformatter(_RSAOAEPKeyExchangeDeformatterVtbl): IDispatch(IDispatchVtbl)  
{}}