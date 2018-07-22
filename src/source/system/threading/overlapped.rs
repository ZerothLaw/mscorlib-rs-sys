use winapi::ctypes::c_long;

use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//struct __declspec(uuid("a2959123-2f66-35b4-815d-37c83360809b"))
STRUCT!{struct NativeOverlapped {
    InternalLow: c_long,
    InternalHigh: c_long,
    OffsetLow: c_long,
    OffsetHigh: c_long,
    EventHandle: c_long,
}}

RIDL!{#[uuid(0xdd846fcc, 0x8d04, 0x3665, 0x81, 0xb6, 0xaa, 0xcb, 0xe9, 0x9c, 0x19, 0xc3)]
interface _Overlapped(_OverlappedVtbl): IDispatch(IDispatchVtbl)  
{}} //