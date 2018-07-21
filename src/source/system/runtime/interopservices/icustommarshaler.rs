use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::ctypes::c_long;
use winapi::um::oaidl::VARIANT;
use system::intptr::IntPtr;

RIDL!{#[uuid(0x601cd486, 0x04bf, 0x3213, 0x9e, 0xa9, 0x06, 0xeb, 0xe4, 0x35, 0x1d, 0x74)]
interface ICustomMarshaler(ICustomMarshalerVtbl) : IDispatch(IDispatchVtbl) {
    fn MarshalNativeToManaged(
        pNativeData: IntPtr, 
        pRetVal: *mut VARIANT, 
    ) -> HRESULT, 

    fn MarshalManagedToNative(
        ManagedObj: VARIANT, 
        pRetVal: *mut IntPtr,
    ) -> HRESULT,

    fn CleanUpNativeData(
        pNativeData: IntPtr, 
    ) -> HRESULT, 

    fn CleanUpManagedData(
        ManagedObj: VARIANT, 
    ) -> HRESULT,

    fn GetNativeDataSize(
        pRetVal: *mut c_long,
    ) -> HRESULT,

}}