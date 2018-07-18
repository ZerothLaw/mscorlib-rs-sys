use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xb45bbd7e, 0xa977, 0x3f56, 0xa6, 0x26, 0x7a, 0x69, 0x3e, 0x5d, 0xbb, 0xc5)]
interface _ThreadStart(_ThreadStartVtbl): IDispatch(IDispatchVtbl)  
{}} //delegate