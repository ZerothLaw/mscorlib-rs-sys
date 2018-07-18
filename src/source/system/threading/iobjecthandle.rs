use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::VARIANT;

RIDL!{#[uuid(0xc460e2b4, 0xe199, 0x412a, 0x84, 0x56, 0x84, 0xdc, 0x3e, 0x48, 0x38, 0xc3)]
interface IObjectHandle(IObjectHandleVtbl) : IUnknown(IUnknownVtbl) {
    fn Unwrap(
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
}}