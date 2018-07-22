use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xddedb94d, 0x4f3f, 0x35c1, 0x97, 0xc9, 0x3f, 0x1d, 0x87, 0x62, 0x8d, 0x9e)]
interface _Encoding(_EncodingVtbl): IDispatch(IDispatchVtbl)  
{}} //IClonable