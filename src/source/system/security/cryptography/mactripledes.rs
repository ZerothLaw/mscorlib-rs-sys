use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x1cac0bda, 0xac58, 0x31bc, 0xb6, 0x24, 0x63, 0xf7, 0x7d, 0x0c, 0x3d, 0x2f)]
interface _MACTripleDES(_MACTripleDESVtbl): IDispatch(IDispatchVtbl)  
{}} //KeyedHashAlgorithm
