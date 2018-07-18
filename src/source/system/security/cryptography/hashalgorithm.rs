use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x69d3baba, 0x1c3d, 0x354c, 0xac, 0xfe, 0xf1, 0x91, 0x09, 0xec, 0x38, 0x96)]
interface _HashAlgorithm(_HashAlgorithmVtbl): IDispatch(IDispatchVtbl)  
{}} //IDisposable, ICryptoTransform