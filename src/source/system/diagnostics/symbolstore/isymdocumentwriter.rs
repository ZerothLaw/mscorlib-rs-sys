use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::SAFEARRAY;
use winapi::shared::guiddef::GUID;

RIDL!{#[uuid(0xfa682f24, 0x3a3c, 0x390d, 0xb8, 0xa2, 0x96, 0xf1, 0x10, 0x6f, 0x4b, 0x37)]
interface ISymbolDocumentWriter(ISymbolDocumentWriterVtbl) : IDispatch(IDispatchVtbl) {
    fn SetSource(
        Source: *mut SAFEARRAY,
    ) -> HRESULT,

    fn SetCheckSum(
        algorithmId: GUID, 
        checkSum: *mut SAFEARRAY,
    ) -> HRESULT,
}}
