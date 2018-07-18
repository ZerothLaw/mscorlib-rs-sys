use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xab538809, 0x3c2f, 0x35d9, 0x80, 0xe6, 0x7b, 0xad, 0x54, 0x04, 0x84, 0xa1)]
interface _Stack(_StackVtbl): IDispatch(IDispatchVtbl)  
{}} //ICollection, ICloneable