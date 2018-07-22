use winapi::um::oaidl::{VARIANT};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};


use winapi::ctypes::c_long;
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::VARIANT_BOOL;

RIDL!{#[uuid(0xaab7c6ea, 0xcab0, 0x3adb, 0x82, 0xaa, 0xcf, 0x32, 0xe2, 0x9a, 0xf2, 0x3)]
interface IEqualityComparer(IEqualityComparerVtbl) : IDispatch(IDispatchVtbl){
    fn Equals(
        x: VARIANT, 
        y: VARIANT, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetHashCode(
        obj: VARIANT, 
        pRetVal: *mut c_long,
    ) -> HRESULT,
}}
