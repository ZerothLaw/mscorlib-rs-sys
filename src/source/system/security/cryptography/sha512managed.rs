use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xdc8ce439, 0x7954, 0x36ed, 0x80, 0x3c, 0x67, 0x4f, 0x72, 0xf2, 0x72, 0x49)]
interface _SHA512Managed(_SHA512ManagedVtbl): IDispatch(IDispatchVtbl)  
{}} //SHA512