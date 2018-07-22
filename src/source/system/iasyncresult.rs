use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::um::oaidl::VARIANT;

 use system::threading::_WaitHandle;

RIDL!{#[uuid(0x11ab34e7, 0x0176, 0x3c9e, 0x9e, 0xfe, 0x19, 0x78, 0x58, 0x40, 0x0a, 0x3d)]
interface IAsyncResult(IAsyncResultVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_IsCompleted(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_AsyncWaitHandle(
			pRetVal: *mut *mut _WaitHandle,
	) -> HRESULT,
    fn get_AsyncState(
        pRetVal: *mut *mut VARIANT,
    ) -> HRESULT,
      fn get_CompletedSynchronously(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
}}