use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x9e230640, 0xa5d0, 0x30e1, 0xb2, 0x17, 0x9d, 0x2b, 0x6c, 0xc0, 0xfc, 0x40)]
interface _OperatingSystem(_OperatingSystemVtbl): IDispatch(IDispatchVtbl)  
{}} //ICloneable , ISerializable