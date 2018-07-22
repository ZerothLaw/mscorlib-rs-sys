use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x2b67cece, 0x71c3, 0x36a9, 0xa1, 0x36, 0x92, 0x5c, 0xcc, 0x19, 0x35, 0xa8)]
interface _Array(_ArrayVtbl): IDispatch(IDispatchVtbl)  
{}} //ICloneable, IList, IStructuralComparable, IStructuralEquatable 