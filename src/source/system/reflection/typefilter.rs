use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xe1817846, 0x3745, 0x3c97, 0xb4, 0xa6, 0xee, 0x20, 0xa1, 0x64, 0x1b, 0x29)]
interface _TypeFilter(_TypeFilterVtbl): IDispatch(IDispatchVtbl)  
{}} //public delegate bool TypeFilter(Type m, Object filterCriteria);