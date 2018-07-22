use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::{BSTR};
use winapi::um::oaidl::SAFEARRAY;

RIDL!{#[uuid(0xe699146c, 0x7793, 0x3455, 0x9b, 0xef, 0x96, 0x4c, 0x90, 0xd8, 0xf9, 0x95)]
interface ISoapMessage(ISoapMessageVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_ParamNames(
	    pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn put_ParamNames(
        pRetVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_ParamValues(
	    pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn put_ParamValues(
        pRetVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_ParamTypes(
	    pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn put_ParamTypes(
        pRetVal: *mut SAFEARRAY,
    ) -> HRESULT,
      fn get_MethodName(
        pRetVal: *mut BSTR ,
    ) -> HRESULT,
      fn put_MethodName(
        pRetVal: BSTR,
    ) -> HRESULT,
    fn get_XmlNameSpace(
        pRetVal: *mut BSTR ,
    ) -> HRESULT,
    fn put_XmlNameSpace(
        pRetVal: BSTR,
    ) -> HRESULT,
    fn get_headers(
	    pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn put_headers(
        pRetVal: *mut SAFEARRAY,
    ) -> HRESULT,
}}
