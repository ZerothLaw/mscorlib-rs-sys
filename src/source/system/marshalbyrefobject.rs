use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x2c358e27, 0x8c1a, 0x3c03, 0xb0, 0x86, 0xa4, 0x04, 0x65, 0x62, 0x55, 0x57)]
interface _MarshalByRefObject(_MarshalByRefObjectVtbl): IDispatch(IDispatchVtbl)  
{}}