use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x2c65d4c0, 0x584c, 0x3e4e, 0x8e, 0x6d, 0x1a, 0xfb, 0x11, 0x2b, 0xff, 0x69)]
interface _RNGCryptoServiceProvider(_RNGCryptoServiceProviderVtbl): IDispatch(IDispatchVtbl)  
{}} //RandomNumberGenerator