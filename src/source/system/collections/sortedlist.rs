use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x56421139, 0xa143, 0x3ae9, 0x98, 0x52, 0x1d, 0xbd, 0xfe, 0x3d, 0x6b, 0xfa)]
interface _SortedList(_SortedListVtbl): IDispatch(IDispatchVtbl)  
{}} //IDictionary, ICloneable Obsolete