use winapi::ctypes::c_long;
use winapi::shared::winerror::HRESULT;

use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY, VARIANT};

use unknown::_Type;
use dispatch::_SerializationInfo;
use unknown::_MethodInfo;

use source::system::runtime::serialization::streamingcontext::StreamingContext;

RIDL!{#[uuid(0xfb6ab00f, 0x5096, 0x3af8, 0xa3, 0x3d, 0xd7, 0x88, 0x5a, 0x5f, 0xa8, 0x29)]
interface _Delegate(_DelegateVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_ToString(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn Equals(
        obj: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetHashCode(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn GetType(
		pRetVal: *mut *mut  _Type ,
	) -> HRESULT,
    fn GetInvocationList(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn Clone_(
        pRetVal: VARIANT ,
    ) -> HRESULT,
    fn GetObjectData(
        info: *mut _SerializationInfo, 
        Context: StreamingContext,
    ) -> HRESULT,
    fn DynamicInvoke(
		args: *mut SAFEARRAY ,
		pRetVal: *mut VARIANT ,
	) -> HRESULT,
    fn get_Method(
		pRetVal: *mut *mut  _MethodInfo ,
	) -> HRESULT,
    fn get_Target(
        pRetVal: VARIANT ,
    ) -> HRESULT,
}}//implements ICloneable, ISerializable