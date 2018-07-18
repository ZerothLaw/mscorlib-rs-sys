use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xb7d29e26, 0x7798, 0x3fa4, 0x90, 0xf4, 0xe6, 0xa2, 0x2d, 0x20, 0x99, 0xf9)]
interface _CollectionBase(_CollectionBaseVtbl): IDispatch(IDispatchVtbl)  
{}} //IList