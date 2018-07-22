use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x4c482cc2, 0x68e9, 0x37c6, 0x83, 0x53, 0x9a, 0x94, 0xbd, 0x2d, 0x7f, 0x0b)]
interface _SystemException(_SystemExceptionVtbl): IDispatch(IDispatchVtbl)  
{}} //Exception