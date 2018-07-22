use winapi::um::oaidl::{VARIANT};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};


use winapi::ctypes::c_long;
use winapi::shared::winerror::HRESULT;

RIDL!{#[uuid(0xc20fd3eb, 0x7022, 0x3d14, 0x84, 0x77, 0x76, 0x0f, 0xab, 0x54, 0xe5, 0x0d)]
interface IComparer(IComparerVtbl) : IDispatch(IDispatchVtbl)
{
    fn Compare(
        x: VARIANT, 
        y: VARIANT, 
        pRetVal: *mut c_long,
    ) -> HRESULT,
}}
