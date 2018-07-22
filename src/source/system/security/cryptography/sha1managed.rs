use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xc27990bb, 0x3cfd, 0x3d29, 0x8d, 0xc0, 0xbb, 0xe5, 0xfb, 0xad, 0xea, 0xfd)]
interface _SHA1Managed(_SHA1ManagedVtbl): IDispatch(IDispatchVtbl)  
{}} //SHA1