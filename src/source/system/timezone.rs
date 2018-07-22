use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x3025f666, 0x7891, 0x33d7, 0xaa, 0xcd, 0x23, 0xd1, 0x69, 0xef, 0x35, 0x4e)]
interface _TimeZone(_TimeZoneVtbl): IDispatch(IDispatchVtbl)  
{}} //deprecated