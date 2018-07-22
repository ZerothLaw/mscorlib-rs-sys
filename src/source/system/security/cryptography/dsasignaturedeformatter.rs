use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x0e774498, 0xade6, 0x3820, 0xb1, 0xd5, 0x42, 0x6b, 0x06, 0x39, 0x7b, 0xe7)]
interface _DSASignatureDeformatter(_DSASignatureDeformatterVtbl): IDispatch(IDispatchVtbl)  
{}} // AsymmetricSignatureDeformatter 