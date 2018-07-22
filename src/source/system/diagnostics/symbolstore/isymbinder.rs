use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::BSTR;
use winapi::ctypes::c_long;

use system::diagnostics::symbolstore::isymreader::ISymbolReader;

RIDL!{#[uuid(0x20808adc, 0xcc01, 0x3f3a, 0x8f, 0x09, 0xed, 0x12, 0x94, 0x0f, 0xc2, 0x12)]
interface ISymbolBinder(ISymbolBinderVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetReader(
        importer: c_long, 
        filename: BSTR, 
        searchPath: BSTR, 
        pRetVal: *mut *mut ISymbolReader, 
    ) -> HRESULT,
}}//obsolete

RIDL!{#[uuid(0x027c036a, 0x4052, 0x3821, 0x85, 0xde, 0xb5, 0x33, 0x19, 0xdf, 0x12, 0x11)]
interface ISymbolBinder1(ISymbolBinder1Vtbl): IDispatch(IDispatchVtbl)  
{
    fn GetReader(
        importer: c_long, 
        filename: BSTR, 
        searchPath: BSTR, 
        pRetVal: *mut *mut ISymbolReader, 
    ) -> HRESULT,
}}