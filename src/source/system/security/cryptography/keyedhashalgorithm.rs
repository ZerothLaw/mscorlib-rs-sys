use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xd182cf91, 0x628c, 0x3ff6, 0x87, 0xf0, 0x41, 0xba, 0x51, 0xcc, 0x74, 0x33)]
interface _KeyedHashAlgorithm(_KeyedHashAlgorithmVtbl): IDispatch(IDispatchVtbl)  
{}} //HashAlgorithm