use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x425bff0d, 0x59e4, 0x36a8, 0xb1, 0xff, 0x1f, 0x5d, 0x39, 0xd6, 0x98, 0xf4)]
interface _PKCS1MaskGenerationMethod(_PKCS1MaskGenerationMethodVtbl): IDispatch(IDispatchVtbl)  
{}} //MaskGenerationMethod