use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::LPSAFEARRAY;
use winapi::shared::wtypes::BSTR;

RIDL!{#[uuid(0x23ed2454, 0x6899, 0x3c28, 0xba, 0xb7, 0x6e, 0xc8, 0x66, 0x83, 0x96, 0x4a)]
interface ISymbolNamespace(ISymbolNamespaceVtbl) : IDispatch(IDispatchVtbl) {
    fn get_name(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn GetNamespaces(
        pRetVal: *mut LPSAFEARRAY, 
    ) -> HRESULT, 
    fn GetVariables(
        pRetVal: *mut LPSAFEARRAY, 
    ) -> HRESULT,
}}