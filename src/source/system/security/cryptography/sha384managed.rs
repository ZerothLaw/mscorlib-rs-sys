use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xde541460, 0xf838, 0x3698, 0xb2, 0xda, 0x51, 0x0b, 0x09, 0x07, 0x01, 0x18)]
interface _SHA384Managed(_SHA384ManagedVtbl): IDispatch(IDispatchVtbl)  
{}} //sha384