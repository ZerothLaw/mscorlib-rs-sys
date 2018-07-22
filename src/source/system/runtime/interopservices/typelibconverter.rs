use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xf1c3bf79, 0xc3e4, 0x11d3, 0x88, 0xe7, 0x00, 0x90, 0x27, 0x54, 0xc4, 0x3a)]
interface _TypeLibConverter(_TypeLibConverterVtbl): IDispatch(IDispatchVtbl){
}}// ITypeLibConverter