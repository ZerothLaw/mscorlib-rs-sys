use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x0d9f1b65, 0x6d27, 0x3e9f, 0xba, 0xf3, 0x05, 0x97, 0x83, 0x7e, 0x0f, 0x33)]
interface _DBNull(_DBNullVtbl): IDispatch(IDispatchVtbl)  
{}} //ISerializable, IConvertible
