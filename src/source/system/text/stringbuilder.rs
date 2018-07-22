use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x9fb09782, 0x8d39, 0x3b0c, 0xb7, 0x9e, 0xf7, 0xa3, 0x7a, 0x65, 0xb3, 0xda)]
interface _StringBuilder(_StringBuilderVtbl): IDispatch(IDispatchVtbl)  
{}} //ISerializable