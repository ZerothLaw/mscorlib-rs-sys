use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x4ca8147e, 0xbaa3, 0x3a7f, 0x92, 0xce, 0xa4, 0xfd, 0x7f, 0x17, 0xd8, 0xda)]
interface _BinaryWriter(_BinaryWriterVtbl): IDispatch(IDispatchVtbl)  
{}} //IDisposable
