use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x8017b414, 0x4886, 0x33da, 0x80, 0xa3, 0x78, 0x65, 0xc1, 0x35, 0x0d, 0x43)]
interface _SignatureDescription(_SignatureDescriptionVtbl): IDispatch(IDispatchVtbl)  
{}}