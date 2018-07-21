use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::ctypes::c_long;
use winapi::shared::wtypes::BSTR;
use winapi::um::oaidl::SAFEARRAY;
use winapi::shared::guiddef::GUID;
use winapi::shared::wtypes::VARIANT_BOOL;

use system::diagnostics::symbolstore::isymdocumentwriter::ISymbolDocumentWriter;
use system::diagnostics::symbolstore::token::SymbolToken;
use system::diagnostics::symbolstore::symaddresskind::SymAddressKind;
use system::intptr::IntPtr;

use system::reflection::fieldattributes::FieldAttributes;
use system::reflection::parameterattributes::ParameterAttributes;

RIDL!{#[uuid(0xda295a1b, 0xc5bd, 0x3b34, 0x8a, 0xcd, 0x1d, 0x7d, 0x33, 0x4f, 0xfb, 0x7f)]
interface ISymbolWriter(ISymbolWriterVtbl) : IDispatch(IDispatchVtbl) {
    fn Initialize(
        emitter: IntPtr, 
        filename: BSTR, 
        fFullBuild: VARIANT_BOOL,
    ) -> HRESULT,
    fn DefineDocument(
        url: BSTR, 
        Language: GUID, 
        LanguageVendor: GUID, 
        DocumentType: GUID, 
        pRetVal: *mut *mut ISymbolDocumentWriter,
    ) -> HRESULT,
    fn SetUserEntryPoint(
        entryMethod: *mut SymbolToken,
    ) -> HRESULT,
    fn OpenMethod(
        Method: *mut SymbolToken,
    ) -> HRESULT, 
    fn CloseMethod() -> HRESULT,
    fn DefineSequencePoints(
        document: *mut ISymbolDocumentWriter,
        offsets: *mut SAFEARRAY, //int[]
        lines: *mut SAFEARRAY, //int[]
        columns: *mut SAFEARRAY, //int[]
        endLine: *mut SAFEARRAY, //int[]
        endColumns: *mut SAFEARRAY,//int[]
    ) -> HRESULT,
    fn OpenScope(
        StartOffset: c_long,
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn CloseScope(
        EndOffset: c_long,
    ) -> HRESULT,
    fn SetScopeRange(
        scopeID: c_long, 
        StartOffset: c_long, 
        EndOffset: c_long,
    ) -> HRESULT,
    fn DefineLocalVariable(
        name: BSTR, 
        Attributes: FieldAttributes, 
        signaure: *mut SAFEARRAY, //byte[]
        addrKind: SymAddressKind,
        addr1: c_long, 
        addr2: c_long, 
        addr3: c_long, 
        StartOffset: c_long, 
        EndOffset: c_long,
    ) -> HRESULT,
    fn DefineParameter(
        name: BSTR, 
        Attributes: ParameterAttributes, 
        sequence: c_long, 
        addrKind: SymAddressKind,
        addr1: c_long, 
        addr2: c_long,
        addr3: c_long,
    ) -> HRESULT,
    fn DefineField(
        parent: SymbolToken, 
        name: BSTR, 
        Attributes: FieldAttributes, 
        signature: *mut SAFEARRAY, //byte[] 
        addrKind: SymAddressKind, 
        addr1: c_long, 
        addr2: c_long, 
        addr3: c_long,
    ) -> HRESULT,
    fn DefineGlobalVariable(
        name: BSTR, 
        Attributes: FieldAttributes, 
        signature: *mut SAFEARRAY, //byte[]
        addrKind: SymAddressKind, 
        addr1: c_long, 
        addr2: c_long, 
        addr3: c_long,
    ) -> HRESULT,
    fn Close() -> HRESULT,
    fn SetSymAttribute(
        parent: SymbolToken, 
        name: BSTR, 
        data: *mut SAFEARRAY, //byte[]
    ) -> HRESULT,
    fn OpenNamespace(
        name: BSTR,
    ) -> HRESULT,
    fn CloseNamespace() -> HRESULT,
    fn UsingNamespace(
        FullName: BSTR, 
    ) -> HRESULT,
    fn SetMethodSourceRange(
        startDoc: *mut ISymbolDocumentWriter, 
        startLine: c_long, 
        startColumn: c_long, 
        endDoc: *mut ISymbolDocumentWriter, 
        endLine: c_long, 
        endColumn: c_long, 
    ) -> HRESULT,
    fn SetUnderlyingWriter(
        underlyingWriter: c_long, //IntPtr?
    ) -> HRESULT,
}}