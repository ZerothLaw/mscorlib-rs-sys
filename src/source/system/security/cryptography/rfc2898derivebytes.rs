use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xa6589897, 0x5a67, 0x305f, 0x94, 0x97, 0x72, 0xe5, 0xfe, 0x8b, 0xea, 0xd5)]
interface _Rfc2898DeriveBytes(_Rfc2898DeriveBytesVtbl): IDispatch(IDispatchVtbl)  
{}} //DeriveBytes
