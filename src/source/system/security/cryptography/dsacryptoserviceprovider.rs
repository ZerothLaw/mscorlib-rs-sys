use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x1f38aafe, 0x7502, 0x332f, 0x97, 0x1f, 0xc2, 0xfc, 0x70, 0x0a, 0x1d, 0x55)]
interface _DSACryptoServiceProvider(_DSACryptoServiceProviderVtbl): IDispatch(IDispatchVtbl)  
{}} //implements DSA, ICspAssymetricAlgorithm