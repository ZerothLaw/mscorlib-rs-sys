use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x44494e35, 0xc370, 0x3014, 0xbc, 0x78, 0x0f, 0x2e, 0xcb, 0xf8, 0x3f, 0x53)]
interface _PolicyLevel(_PolicyLevelVtbl): IDispatch(IDispatchVtbl)  
{}}