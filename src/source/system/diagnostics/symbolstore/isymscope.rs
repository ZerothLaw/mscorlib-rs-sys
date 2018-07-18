use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::SAFEARRAY;
use winapi::ctypes::c_long;   

use source::system::diagnostics::symbolstore::isymmethod::ISymbolMethod;

RIDL!{#[uuid(0x1cee3a11, 0x01ae, 0x3244, 0xa9, 0x39, 0x49, 0x72, 0xfc, 0x97, 0x03, 0xef)]
interface ISymbolScope(ISymbolScopeVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_Method(
		pRetVal: *mut *mut  ISymbolMethod,
	) -> HRESULT,
    fn get_parent(
		pRetVal: *mut *mut  ISymbolScope,
	) -> HRESULT,
    fn GetChildren(
		pRetVal: *mut *mut SAFEARRAY, //ISymbolScope[]
	) -> HRESULT,
    fn get_StartOffset(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn get_EndOffset(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn GetLocals(
		pRetVal: *mut *mut SAFEARRAY, //ISymbolVariable[]
	) -> HRESULT,
    fn GetNamespaces(
		pRetVal: *mut *mut SAFEARRAY, //ISymbolNamespace[]
	) -> HRESULT,
}}