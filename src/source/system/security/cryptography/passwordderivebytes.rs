use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x3cd62d67, 0x586f, 0x309e, 0xa6, 0xd8, 0x1f, 0x4b, 0xaa, 0xc5, 0xac, 0x28)]
interface _PasswordDeriveBytes(_PasswordDeriveBytesVtbl): IDispatch(IDispatchVtbl)  
{}} //DeriveBytes