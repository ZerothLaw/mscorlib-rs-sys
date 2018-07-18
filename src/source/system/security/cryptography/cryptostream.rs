use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("8990cb3b-227e-3a43-8264-0057ec763fa0"))
ENUM!{enum CryptoStreamMode
{
    CryptoStreamMode_Read = 0,
    CryptoStreamMode_Write = 1,
}}

RIDL!{#[uuid(0x4134f762, 0xd0ec, 0x3210, 0x93, 0xc0, 0xde, 0x4f, 0x44, 0x3d, 0x56, 0x69)]
interface _CryptoStream(_CryptoStreamVtbl): IDispatch(IDispatchVtbl)  
{}} //Stream, IDisposable