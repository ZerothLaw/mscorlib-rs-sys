use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x3d077954, 0x7bcc, 0x325b, 0x9d, 0xda, 0x3b, 0x17, 0xa0, 0x33, 0x78, 0xe0)]
interface _SHA256Managed(_SHA256ManagedVtbl): IDispatch(IDispatchVtbl)  
{}} //SHA256