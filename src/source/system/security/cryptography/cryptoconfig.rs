use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xab00f3f8, 0x7dde, 0x3ff5, 0xb8, 0x05, 0x6c, 0x5d, 0xbb, 0x20, 0x05, 0x49)]
interface _CryptoConfig(_CryptoConfigVtbl): IDispatch(IDispatchVtbl)  
{}}