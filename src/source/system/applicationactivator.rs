use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xd1204423, 0x01f0, 0x336a, 0x89, 0x11, 0xa7, 0xe8, 0xfb, 0xe1, 0x85, 0xa3)]
interface _ApplicationActivator(_ApplicationActivatorVtbl): IDispatch(IDispatchVtbl)  
{}}
