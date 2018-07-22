use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xdfad74dc, 0x8390, 0x32f6, 0x96, 0x12, 0x1b, 0xd2, 0x93, 0xb2, 0x33, 0xf4)]
interface _FileCodeGroup(_FileCodeGroupVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeGroup, IUnionSemanticCodeGroup 