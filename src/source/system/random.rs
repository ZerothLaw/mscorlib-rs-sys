use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x0f240708, 0x629a, 0x31ab, 0x94, 0xa5, 0x2b, 0xb4, 0x76, 0xfe, 0x17, 0x83)]
interface _Random(_RandomVtbl): IDispatch(IDispatchVtbl)  
{}}