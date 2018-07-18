use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x5767c78f, 0xf344, 0x35a5, 0x84, 0xbc, 0x53, 0xb9, 0xea, 0xeb, 0x68, 0xcb)]
interface _RijndaelManagedTransform(_RijndaelManagedTransformVtbl): IDispatch(IDispatchVtbl)  
{}} //ICryptoTransform
