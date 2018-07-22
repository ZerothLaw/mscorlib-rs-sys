
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::VARIANT_BOOL;

RIDL!{#[uuid(0x496b0abf, 0xcdee, 0x11d3, 0x88, 0xe8, 0x00, 0x90, 0x27, 0x54, 0xc4, 0x3a)]
interface IEnumerator(IEnumeratorVtbl) : IDispatch(IDispatchVtbl){
    fn MoveNext(
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Current(
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
}}