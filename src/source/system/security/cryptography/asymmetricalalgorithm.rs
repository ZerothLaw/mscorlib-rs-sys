use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x09343ac0, 0xd19a, 0x3e62, 0xbc, 0x16, 0x0f, 0x60, 0x0f, 0x10, 0x18, 0x0a)]
interface _AsymmetricAlgorithm(_AsymmetricAlgorithmVtbl): IDispatch(IDispatchVtbl)  
{}} //implements IDisposable
