use winapi::ctypes::c_long;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::BSTR;
use winapi::um::oaidl::{VARIANT, SAFEARRAY};
use winapi::shared::wtypes::VARIANT_BOOL;

use source::system::reflection::cominterfaces::_MethodBase;
use dispatch::_LogicalCallContext;

RIDL!{#[uuid(0x8e5e0b95, 0x750e, 0x310d, 0x89, 0x2c, 0x8c, 0xa7, 0x23, 0x1c, 0xf7, 0x5b)]
interface IMethodMessage(IMethodMessageVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_Uri(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_MethodName(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_typeName(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_MethodSignature(
        pRetVal: VARIANT ,
    ) -> HRESULT,
    fn get_ArgCount(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn GetArgName(
        index: c_long,
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn GetArg(
        argNum: c_long,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn get_args(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn get_HasVarArgs(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_LogicalCallContext(
		pRetVal: *mut *mut  _LogicalCallContext ,
	) -> HRESULT,
    fn get_MethodBase(
		pRetVal: *mut *mut  _MethodBase ,
	) -> HRESULT,
}}