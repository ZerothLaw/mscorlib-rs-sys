use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::um::oaidl::SAFEARRAY;
use winapi::ctypes::c_long;

use system::diagnostics::symbolstore::isymnamespace::ISymbolNamespace;
use system::diagnostics::symbolstore::isymdocument::ISymbolDocument;
use system::diagnostics::symbolstore::isymscope::ISymbolScope;
use system::diagnostics::symbolstore::token::SymbolToken;

RIDL!{#[uuid(0x25c72eb0, 0xe437, 0x3f17, 0x94, 0x6d, 0x3b, 0x72, 0xa3, 0xac, 0xff, 0x37)]
interface ISymbolMethod(ISymbolMethodVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_Token(
        pRetVal: SymbolToken,
    ) -> HRESULT,
    fn get_SequencePointCount(
        pRetVal: c_long,
    ) -> HRESULT,
    fn GetSequencePoints(
        offsets: *mut SAFEARRAY, //int[] 
        documents: *mut SAFEARRAY, //ISymbolDocument[]
        lines: *mut SAFEARRAY, //int[]
        columns: *mut SAFEARRAY, //int[]
        endLines: *mut SAFEARRAY,//int[]
        endColumns: *mut SAFEARRAY,//int[]
    ) -> HRESULT, 
    fn get_RootScope(
		pRetVal: *mut *mut  ISymbolScope ,
	) -> HRESULT,
    fn GetScope(
        offset: c_long, 
        pRetVal: *mut *mut ISymbolScope,
    ) -> HRESULT,
    fn GetOffset(
        document: *mut ISymbolDocument, 
        line: c_long, 
        column: c_long, 
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn GetRanges(
        document: *mut ISymbolDocument, 
        line: c_long, 
        column: c_long, 
        pRetVal: *mut *mut SAFEARRAY , 
    ) -> HRESULT,
    fn GetParameters(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn GetNamespace(
		pRetVal: *mut *mut  ISymbolNamespace ,
	) -> HRESULT,
    fn GetSourceStartEnd(
        docs: *mut SAFEARRAY, //ISymbolDocument[]
        lines: *mut SAFEARRAY, //int[]
        columns: *mut SAFEARRAY,  //int[]
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}
