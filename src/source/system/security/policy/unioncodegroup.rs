use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xd9d822de, 0x44e5, 0x33ce, 0xa4, 0x3f, 0x17, 0x3e, 0x47, 0x5c, 0xec, 0xb1)]
interface _UnionCodeGroup(_UnionCodeGroupVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeGroup, IUnionSemanticCodeGroup
//Obsolete