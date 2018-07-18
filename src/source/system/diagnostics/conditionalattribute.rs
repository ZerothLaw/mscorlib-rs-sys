use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xe40a025c, 0x645b, 0x3c8e, 0xa1, 0xac, 0x9c, 0x5c, 0xca, 0x27, 0x96, 0x25)]
interface _ConditionalAttribute(_ConditionalAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //Attribute