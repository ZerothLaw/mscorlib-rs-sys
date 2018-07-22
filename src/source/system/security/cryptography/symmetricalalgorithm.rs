use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x05bc0e38, 0x7136, 0x3825, 0x9e, 0x34, 0x26, 0xc1, 0xcf, 0x21, 0x42, 0xc9)]
interface _SymmetricAlgorithm(_SymmetricAlgorithmVtbl): IDispatch(IDispatchVtbl)  
{}} //abstract, IDisposable