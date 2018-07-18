use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x49dd9e4b, 0x84f3, 0x3d6d, 0x91, 0xfb, 0x3f, 0xed, 0xce, 0xf6, 0x34, 0xc7)]
interface _SHA512(_SHA512Vtbl): IDispatch(IDispatchVtbl)  
{}} //HashAlgorithm

