use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xc5df3568, 0xc251, 0x3c58, 0xaf, 0xb4, 0x32, 0xe7, 0x9e, 0x82, 0x61, 0xf0)]
interface _WeakReference(_WeakReferenceVtbl): IDispatch(IDispatchVtbl)  
{}} //ISerializable