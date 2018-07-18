use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xfc38507e, 0x06a4, 0x3300, 0x86, 0x52, 0x8d, 0x7b, 0x54, 0x34, 0x1f, 0x65)]
interface _RSAPKCS1SignatureDeformatter(_RSAPKCS1SignatureDeformatterVtbl): IDispatch(IDispatchVtbl)  
{}}
