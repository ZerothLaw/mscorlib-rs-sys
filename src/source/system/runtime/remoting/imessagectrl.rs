use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::ctypes::c_long;

RIDL!{#[uuid(0x3677cbb0, 0x784d, 0x3c15, 0xbb, 0xc8, 0x75, 0xcd, 0x7d, 0xc3, 0x90, 0x1e)]
interface IMessageCtrl(IMessageCtrlVtbl): IDispatch(IDispatchVtbl)  
{
    fn Cancel(
        msToCancel: c_long,
    ) -> HRESULT,
}}