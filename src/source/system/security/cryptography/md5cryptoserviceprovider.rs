use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xd3f5c812, 0x5867, 0x33c9, 0x8c, 0xee, 0xcb, 0x17, 0x0e, 0x8d, 0x84, 0x4a)]
interface _MD5CryptoServiceProvider(_MD5CryptoServiceProviderVtbl): IDispatch(IDispatchVtbl)  
{}} //MD5