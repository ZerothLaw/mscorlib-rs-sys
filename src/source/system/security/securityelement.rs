use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x8d597c42, 0x2cfd, 0x32b6, 0xb6, 0xd6, 0x86, 0xc9, 0xe2, 0xcf, 0xf0, 0x0a)]
interface _SecurityElement(_SecurityElementVtbl): IDispatch(IDispatchVtbl)  
{}}