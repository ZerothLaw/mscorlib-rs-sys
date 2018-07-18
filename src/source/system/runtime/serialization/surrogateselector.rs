use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x6de1230e, 0x1f52, 0x3779, 0x96, 0x19, 0xf5, 0x18, 0x41, 0x03, 0x46, 0x6c)]
interface _SurrogateSelector(_SurrogateSelectorVtbl): IDispatch(IDispatchVtbl)  
{}} //implements ISerializationSurrogate