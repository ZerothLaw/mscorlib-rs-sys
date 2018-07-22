use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xb0ad9a21, 0x5439, 0x3d88, 0x89, 0x75, 0x40, 0x18, 0xb8, 0x28, 0xd7, 0x4c)]
interface _LifetimeServices(_LifetimeServicesVtbl): IDispatch(IDispatchVtbl)  
{}}
