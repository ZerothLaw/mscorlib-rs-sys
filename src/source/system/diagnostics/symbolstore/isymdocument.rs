use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::SAFEARRAY;
use winapi::ctypes::c_long;
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::shared::guiddef::GUID;
use winapi::shared::wtypes::BSTR;

RIDL!{#[uuid(0x1c32f012, 0x2684, 0x3efe, 0x8d, 0x50, 0x9c, 0x29, 0x73, 0xac, 0xc0, 0x0b)]
interface ISymbolDocument(ISymbolDocumentVtbl) : IDispatch(IDispatchVtbl) {
    fn get_Url(
        pRetVal: *mut BSTR, 
    ) -> HRESULT,
    fn get_DocumentType(
        pRetVal: *mut GUID, 
    ) -> HRESULT,
    fn get_Language(
        pRetVal: *mut GUID,
    ) -> HRESULT,
    fn get_LanguageVendor(
        pRetVal: *mut GUID, 
    ) -> HRESULT,
    fn get_CheckSumAlgorithmId(
        pRetVal: *mut GUID,
    ) -> HRESULT,
    fn GetCheckSum(
        pRetVal: *mut *mut SAFEARRAY, //byte[]
    ) -> HRESULT,
    fn FindClosestLine(
        line: c_long, 
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn get_HasEmbeddedSource(
        pRetVal: *mut VARIANT_BOOL, 
    ) -> HRESULT,
    fn get_SourceLength(
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn GetSourceRange(
        startLine: c_long, 
        startColumn: c_long, 
        endLine: c_long, 
        endColumn: c_long,
        pRetVal: *mut *mut SAFEARRAY, //byte[]
    ) -> HRESULT,
}} 