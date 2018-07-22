use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xc0bb9361, 0x268f, 0x3e72, 0xbf, 0x6f, 0x41, 0x20, 0x17, 0x5a, 0x15, 0x00)]
interface _ManualResetEvent(_ManualResetEventVtbl): IDispatch(IDispatchVtbl)  
{}} //EventWaitHandle
