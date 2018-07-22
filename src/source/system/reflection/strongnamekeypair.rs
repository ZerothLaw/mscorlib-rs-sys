use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xfc4963cb, 0xe52b, 0x32d8, 0xa4, 0x18, 0xd0, 0x58, 0xfa, 0x51, 0xa1, 0xfa)]
interface _StrongNameKeyPair(_StrongNameKeyPairVtbl): IDispatch(IDispatchVtbl)  
{}}