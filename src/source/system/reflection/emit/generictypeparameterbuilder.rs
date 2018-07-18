use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xb1a62835, 0xfc19, 0x35a4, 0xb2, 0x06, 0xa4, 0x52, 0x46, 0x3d, 0x7e, 0xe7)]
interface _GenericTypeParameterBuilder(_GenericTypeParameterBuilderVtbl): IDispatch(IDispatchVtbl)  
{}} //inherits from TypeInfo