use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xb9e07599, 0x7c44, 0x33be, 0xa7, 0x0e, 0xef, 0xa1, 0x6f, 0x51, 0xf5, 0x4a)]
interface _ThreadInterruptedException(_ThreadInterruptedExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}