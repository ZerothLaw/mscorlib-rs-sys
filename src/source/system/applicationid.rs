use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x2f218f95, 0x4215, 0x3cc6, 0x8a, 0x51, 0xbd, 0x27, 0x70, 0xc0, 0x90, 0xe4)]
interface _ApplicationId(_ApplicationIdVtbl): IDispatch(IDispatchVtbl)  
{}}