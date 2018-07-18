use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x010fc1d0, 0x3ef9, 0x3f3b, 0xaa, 0x0a, 0xb7, 0x8a, 0x1f, 0xf8, 0x3a, 0x37)]
interface _UTF8Encoding(_UTF8EncodingVtbl): IDispatch(IDispatchVtbl)  
{}} //Encoding