use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xfb7a5ff4, 0xcfa8, 0x3f24, 0xad, 0x5f, 0xd5, 0xeb, 0x39, 0x35, 0x97, 0x07)]
interface _RSAPKCS1SignatureFormatter(_RSAPKCS1SignatureFormatterVtbl): IDispatch(IDispatchVtbl)  
{}}