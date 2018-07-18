use winapi::um::oaidl::{VARIANT};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};


use winapi::ctypes::c_long;
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::VARIANT_BOOL;
RIDL!{#[uuid(0x7bcfa00f, 0xf764, 0x3113, 0x91, 0x40, 0x3b, 0xbd, 0x12, 0x7a, 0x96, 0xbb)]
interface IList(IListVtbl) : IDispatch(IDispatchVtbl){
    fn get_Item(
        index: c_long, 
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn putref_Item(
        index: c_long, 
        pRetVal: VARIANT, 
    ) -> HRESULT,
    fn Add(
        value: VARIANT, 
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn Contains(
        value: VARIANT, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Clear() -> HRESULT,
    fn get_IsReadOnly(
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFixedSize(
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn IndexOf(
        value: VARIANT, 
        pRetVal: *mut c_long, 
    ) -> HRESULT,
    fn Insert(
        index: c_long, 
        value: VARIANT, 
    ) -> HRESULT,
    fn Remove(
        value: VARIANT, 
    ) -> HRESULT,
    fn RemoveAt(
        index: c_long,
    ) -> HRESULT,
}} //inherits from ICollection