use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x4b5fc561, 0x5983, 0x31e4, 0x90, 0x3b, 0x14, 0x04, 0x23, 0x1b, 0x2c, 0x89)]
interface _DSASignatureFormatter(_DSASignatureFormatterVtbl): IDispatch(IDispatchVtbl)  
{}} //AsymmetricSignatureFormatter