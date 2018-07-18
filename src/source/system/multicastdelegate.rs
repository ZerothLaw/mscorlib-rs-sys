use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
RIDL!{#[uuid(0x16fe0885, 0x9129, 0x3884, 0xa2, 0x32, 0x90, 0xb5, 0x8c, 0x5b, 0x2a, 0xa9)]
interface _MulticastDelegate(_MulticastDelegateVtbl): IDispatch(IDispatchVtbl)  
{}}
//inherits _Delegate