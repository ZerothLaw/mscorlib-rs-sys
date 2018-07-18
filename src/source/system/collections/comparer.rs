use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x8064a157, 0xb5c8, 0x3a4a, 0xad, 0x3d, 0x02, 0xdc, 0x1a, 0x39, 0xc4, 0x17)]
interface _Comparer(_ComparerVtbl): IDispatch(IDispatchVtbl)  
{}} //IComparer, ISerializable