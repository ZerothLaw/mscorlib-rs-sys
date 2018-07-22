use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x3f243ebd, 0x612f, 0x3db8, 0x9e, 0x03, 0xbd, 0x92, 0x34, 0x3a, 0x83, 0x71)]
interface _AutoResetEvent(_AutoResetEventVtbl): IDispatch(IDispatchVtbl)  
{}} //WaitHandle