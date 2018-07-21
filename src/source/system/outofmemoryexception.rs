use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xcf3edb7e, 0x0574, 0x3383, 0xa4, 0x4f, 0x29, 0x2f, 0x7c, 0x14, 0x5d, 0xb4)]
interface _OutOfMemoryException(_OutOfMemoryExceptionVtbl): IDispatch(IDispatchVtbl)  
{}} //SystemException
