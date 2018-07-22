use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x140ee78f, 0x067f, 0x3765, 0x92, 0x58, 0xc3, 0xbc, 0x72, 0xfe, 0x97, 0x6b)]
interface _DeriveBytes(_DeriveBytesVtbl): IDispatch(IDispatchVtbl)  
{}} //IDisposable