use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x875715c5, 0xcb64, 0x3920, 0x81, 0x56, 0x0e, 0xe9, 0xcb, 0x0e, 0x07, 0xea)]
interface _RC2CryptoServiceProvider(_RC2CryptoServiceProviderVtbl): IDispatch(IDispatchVtbl)  
{}} //RC2