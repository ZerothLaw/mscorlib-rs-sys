use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xc040b889, 0x5278, 0x3132, 0xaf, 0xf9, 0xaf, 0xa6, 0x17, 0x07, 0xa8, 0x1d)]
interface _TripleDES(_TripleDESVtbl): IDispatch(IDispatchVtbl)  
{}} //SymmetricAlgorithm