use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x442e3c03, 0xa205, 0x3f21, 0xaa, 0x4d, 0x31, 0x76, 0x8b, 0xb8, 0xea, 0x28)]
interface _BinaryReader(_BinaryReaderVtbl): IDispatch(IDispatchVtbl)  
{}} //IDisposable