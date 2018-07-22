use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xbd9df856, 0x2300, 0x3254, 0xbc, 0xf0, 0x67, 0x9b, 0xa0, 0x3c, 0x7a, 0x13)]
interface _RSACryptoServiceProvider(_RSACryptoServiceProviderVtbl): IDispatch(IDispatchVtbl)  
{}} //RSA, ICspAsymmetricAlgorithm