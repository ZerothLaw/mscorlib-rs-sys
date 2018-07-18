use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x68fd6f14, 0xa7b2, 0x36c8, 0xa7, 0x24, 0xd0, 0x1f, 0x90, 0xd7, 0x34, 0x77)]
interface _X509Certificate(_X509CertificateVtbl): IDispatch(IDispatchVtbl)  
{}} //IDisposable, IDeserializationCallback, ISerializable