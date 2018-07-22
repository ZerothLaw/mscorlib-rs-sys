use winapi::shared::winerror::HRESULT;

use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x496b0abf, 0xcdee, 0x11d3, 0x88, 0xe8, 0x00, 0x90, 0x27, 0x54, 0xc4, 0x3a)]
interface IDisposable(IDisposableVtbl) : IDispatch(IDispatchVtbl){
    fn Dispose() -> HRESULT,
}}