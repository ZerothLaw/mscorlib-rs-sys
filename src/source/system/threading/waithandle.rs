use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x40dfc50a, 0xe93a, 0x3c08, 0xb9, 0xef, 0xe2, 0xb4, 0xf2, 0x8b, 0x56, 0x76)]
interface _WaitHandle(_WaitHandleVtbl): IDispatch(IDispatchVtbl)  
{}}// MarshalByRefObject, IDisposable