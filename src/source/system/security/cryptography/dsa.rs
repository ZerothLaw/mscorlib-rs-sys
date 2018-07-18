use winapi::ctypes::c_long;
use winapi::um::oaidl::SAFEARRAY;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//struct __declspec(uuid("0c646f46-aa27-350d-88dd-d8c920ce6c2d"))
STRUCT!{struct DSAParameters
{
    P: *mut SAFEARRAY,
    Q: *mut SAFEARRAY,
    G: *mut SAFEARRAY,
    y: *mut SAFEARRAY,
    J: *mut SAFEARRAY,
    x: *mut SAFEARRAY,
    Seed: *mut SAFEARRAY,
    Counter: c_long,
}}


RIDL!{#[uuid(0x0eb5b5e0, 0x1be6, 0x3a5f, 0x87, 0xb3, 0xe3, 0x32, 0x33, 0x42, 0xf4, 0x4e)]
interface _DSA(_DSAVtbl): IDispatch(IDispatchVtbl)  
{}} //AssymetricAlgorithm