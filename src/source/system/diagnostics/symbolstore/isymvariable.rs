use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::ctypes::c_long;
use winapi::um::oaidl::VARIANT;
use winapi::um::oaidl::SAFEARRAY;
use winapi::shared::wtypes::BSTR;

use system::diagnostics::symbolstore::symaddresskind::SymAddressKind;

RIDL!{#[uuid(0x4042bd4d, 0xb5ab, 0x30e8, 0x91, 0x9b, 0x14, 0x91, 0x06, 0x87, 0xba, 0xae)]
interface ISymbolVariable(ISymbolVariableVtbl) : IDispatch(IDispatchVtbl) {
    fn get_name(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Attributes(
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
    fn GetSignature(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_AddressKind(
        pRetVal: *mut SymAddressKind,
    ) -> HRESULT,
    fn get_AddressField1(
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn get_AddressField2(
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn get_AddressField3(
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn get_StartOffset(
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn get_EndOffset(
        pRetVal: *mut c_long,
    ) -> HRESULT,
}}