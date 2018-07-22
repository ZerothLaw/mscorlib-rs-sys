use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xaf53d21a, 0xd6af, 0x3406, 0xb3, 0x99, 0x7d, 0xf9, 0xd2, 0xaa, 0xd4, 0x8a)]
interface _StrongNamePublicKeyBlob(_StrongNamePublicKeyBlobVtbl): IDispatch(IDispatchVtbl)  
{}}