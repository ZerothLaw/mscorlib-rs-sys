use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::ctypes::c_long;
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::um::oaidl::VARIANT;

use dispatch::_SerializationInfo;
use winapi::shared::wtypes::BSTR;

use unknown::_MethodBase;
use unknown::_Type;

use system::runtime::serialization::streamingcontext::StreamingContext;

RIDL!{#[uuid(0xb36b5c63, 0x42ef, 0x38bc, 0xa0, 0x7e, 0x0b, 0x34, 0xc9, 0x8f, 0x16, 0x4a)]
interface _Exception(_ExceptionVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_ToString(
        pRetVal: BSTR,
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
    fn get_Message(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn GetBaseException(
		pRetVal: *mut *mut  _Exception ,
	) -> HRESULT,
    fn get_StackTrace(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_HelpLink(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn put_HelpLink(
        pRetVal: BSTR,
    ) -> HRESULT,
    fn get_Source(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn put_Source(
        pRetVal: BSTR,
    ) -> HRESULT,
    fn GetObjectData(
        info: *mut _SerializationInfo,
        Context: StreamingContext,
    ) -> HRESULT,
    fn get_InnerException(
		pRetVal: *mut *mut  _Exception ,
	) -> HRESULT,
    fn get_TargetSite(
		pRetVal: *mut *mut  _MethodBase ,
	) -> HRESULT,
}}
