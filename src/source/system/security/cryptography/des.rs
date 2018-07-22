use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xc7ef0214, 0xb91c, 0x3799, 0x98, 0xdd, 0xc9, 0x94, 0xaa, 0xbf, 0xc7, 0x41)]
interface _DES(_DESVtbl): IDispatch(IDispatchVtbl)  
{}} //SymmetricAlgorithm
