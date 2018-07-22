use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x65e8495e, 0x5207, 0x3248, 0x92, 0x50, 0x0f, 0xc8, 0x49, 0xb4, 0xf0, 0x96)]
interface _DESCryptoServiceProvider(_DESCryptoServiceProviderVtbl): IDispatch(IDispatchVtbl)  
{}}
