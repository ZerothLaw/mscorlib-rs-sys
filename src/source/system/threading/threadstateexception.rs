use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xa13a41cf, 0xe066, 0x3b90, 0x82, 0xf4, 0x73, 0x10, 0x91, 0x04, 0xe3, 0x48)]
interface _ThreadStateException(_ThreadStateExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}