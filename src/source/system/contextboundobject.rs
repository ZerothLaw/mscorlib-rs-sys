use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x3eb1d909, 0xe8bf, 0x3c6b, 0xad, 0xa5, 0x0e, 0x86, 0xe3, 0x1e, 0x18, 0x6e)]
interface _ContextBoundObject(_ContextBoundObjectVtbl): IDispatch(IDispatchVtbl)  
{}}