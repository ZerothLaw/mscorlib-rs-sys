use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x18e473f6, 0x637b, 0x3c01, 0x8d, 0x46, 0xd0, 0x11, 0xaa, 0xd2, 0x6c, 0x95)]
interface _ApplicationSecurityInfo(_ApplicationSecurityInfoVtbl): IDispatch(IDispatchVtbl)  
{}}