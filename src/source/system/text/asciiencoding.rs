use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x0cbe0204, 0x12a1, 0x3d40, 0x9d, 0x9e, 0x19, 0x5d, 0xe6, 0xaa, 0xa5, 0x34)]
interface _ASCIIEncoding(_ASCIIEncodingVtbl): IDispatch(IDispatchVtbl)  
{}} //Encoding