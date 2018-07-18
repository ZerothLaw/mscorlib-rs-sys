use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xf145c46a, 0xd170, 0x3170, 0xb5, 0x2f, 0x46, 0x78, 0xdf, 0xca, 0x03, 0x00)]
interface _BitArray(_BitArrayVtbl): IDispatch(IDispatchVtbl)  
{}} //ICollection, IClonable