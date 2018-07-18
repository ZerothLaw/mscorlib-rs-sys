use winapi::ctypes::c_long;
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::BSTR;
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::um::oaidl::VARIANT;
use unknown::_Type;

RIDL!{#[uuid(0x65074f7f, 0x63c0, 0x304e, 0xaf, 0x0a, 0xd5, 0x17, 0x41, 0xcb, 0x4a, 0x8d)]
interface _Object(_ObjectVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_ToString(
        pRetVal: BSTR,
    ) -> HRESULT,
    fn Equals(
        obj: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetHashCode(
        pRetVal: c_long,
    ) -> HRESULT,
    fn GetType(
		pRetVal: *mut *mut  _Type ,
	) -> HRESULT,
}}