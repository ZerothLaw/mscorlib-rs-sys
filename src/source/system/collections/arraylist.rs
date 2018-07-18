use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x401f89cb, 0xc127, 0x3041, 0x82, 0xfd, 0xb6, 0x70, 0x35, 0x39, 0x5c, 0x56)]
interface _ArrayList(_ArrayListVtbl): IDispatch(IDispatchVtbl)  
{}} //IList, IClonable