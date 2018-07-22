use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xf174290f, 0xe4cf, 0x3976, 0x88, 0xaa, 0x4f, 0x8e, 0x32, 0xeb, 0x03, 0xdb)]
interface _SecurityException(_SecurityExceptionVtbl): IDispatch(IDispatchVtbl)  
{}} //SystemException