use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::SAFEARRAY;
use winapi::shared::wtypes::BSTR;
use winapi::ctypes::c_long;
use winapi::shared::guiddef::GUID;

use system::diagnostics::symbolstore::token::SymbolToken;
use system::diagnostics::symbolstore::isymmethod::ISymbolMethod;
use system::diagnostics::symbolstore::isymdocument::ISymbolDocument;

RIDL!{#[uuid(0xe809a5f1, 0xd3d7, 0x3144, 0x9b, 0xef, 0xfe, 0x8a, 0xc0, 0x36, 0x46, 0x99)]
interface ISymbolReader(ISymbolReaderVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetDocument(
        Url: BSTR,
        Language: GUID, 
        LanguageVendor: GUID, 
        DocumentType: GUID, 
        pRetVal: *mut *mut ISymbolDocument, 
    ) -> HRESULT,
    fn GetDocuments(
		pRetVal: *mut *mut SAFEARRAY, //ISymbolDocument[]
	) -> HRESULT,
    fn get_UserEntryPoint(
        pRetVal: *mut SymbolToken,
    ) -> HRESULT,
    fn GetMethod(
        Method: SymbolToken, 
        pRetVal: *mut *mut ISymbolMethod,
    ) -> HRESULT,
    fn GetMethod_2(
        Method: SymbolToken, 
        Version: c_long,
    ) -> HRESULT,
    fn GetVariables(
        parent: SymbolToken, 
        pRetVal: *mut *mut SAFEARRAY, //ISymbolVariable[]
    ) -> HRESULT,
    fn GetGlobalVariables(
		pRetVal: *mut *mut SAFEARRAY, //ISymbolVariable[]
	) -> HRESULT,
    fn GetMethodFromDocumentPosition(
        document: *mut ISymbolDocument, 
        line: c_long, 
        column: c_long, 
        pRetVal: *mut *mut ISymbolMethod,
    ) -> HRESULT, 
    fn GetSymAttribute(
        parent: SymbolToken, 
        name: BSTR, 
        pRetVal: *mut *mut SAFEARRAY, //byte[]
    ) -> HRESULT,
    fn GetNamespaces(
		pRetVal: *mut *mut SAFEARRAY, //ISymbolNamespace[]
	) -> HRESULT,
}}