use winapi::ctypes::c_long;

use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::{SAFEARRAY};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

 use system::IntPtr;

RIDL!{#[uuid(0x00020404, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IEnumVARIANT(IEnumVARIANTVtbl): IUnknown(IUnknownVtbl){
    fn Next(
        celt: c_long, 
        rgVar: *mut *mut SAFEARRAY, //object[] MarshalAs(UnmanagedType.LPArray, SizeParamIndex=0)
        pceltFetched: IntPtr,
        pRetVal: *mut c_long, 
    ) -> HRESULT,
    fn Skip(
        celt: c_long, 
        pRetVal: *mut c_long, 
    ) -> HRESULT,
    fn Reset(
        pRetVal: *mut c_long, 
    ) -> HRESULT,
    fn Clone(
        pRetVal: *mut *mut IEnumVARIANT, 
    ) -> HRESULT,
}}