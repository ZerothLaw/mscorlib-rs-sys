use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x0422b845, 0xb636, 0x3688, 0x8f, 0x61, 0x9b, 0x6d, 0x93, 0x09, 0x63, 0x36)]
interface _CaseInsensitiveHashCodeProvider(_CaseInsensitiveHashCodeProviderVtbl): IDispatch(IDispatchVtbl)  
{}}