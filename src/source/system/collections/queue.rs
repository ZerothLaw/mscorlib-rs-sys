use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x3a7d3ca4, 0xb7d1, 0x3a2a, 0x80, 0x0c, 0x8f, 0xc2, 0xac, 0xfc, 0xbd, 0xa4)]
interface _Queue(_QueueVtbl): IDispatch(IDispatchVtbl)  
{}} // ICollection, ICloneable 