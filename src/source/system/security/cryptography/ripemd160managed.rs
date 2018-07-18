use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x814f9c35, 0xb7f8, 0x3ceb, 0x8e, 0x43, 0xe0, 0x1f, 0x09, 0x15, 0x70, 0x60)]
interface _RIPEMD160Managed(_RIPEMD160ManagedVtbl): IDispatch(IDispatchVtbl)  
{}} //RIPEMD160