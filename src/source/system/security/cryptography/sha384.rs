use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xb60ad5d7, 0x2c2e, 0x35b7, 0x8d, 0x77, 0x79, 0x46, 0x15, 0x6c, 0xfe, 0x8e)]
interface _SHA384(_SHA384Vtbl): IDispatch(IDispatchVtbl)  
{}}  //HashAlgorithm