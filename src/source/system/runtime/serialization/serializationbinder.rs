use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x450222d0, 0x87ca, 0x3699, 0xa7, 0xb4, 0xd8, 0xa0, 0xfd, 0xb7, 0x23, 0x57)]
interface _SerializationBinder(_SerializationBinderVtbl): IDispatch(IDispatchVtbl)  
{}}