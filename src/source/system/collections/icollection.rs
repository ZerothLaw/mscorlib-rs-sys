//.Net Framework 4.7.2 Reference Source - mscorlib { system.collections.icollection.cs }

use winapi::ctypes::c_long;
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::VARIANT_BOOL;

use winapi::um::oaidl::{VARIANT};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

use system::_Array;

RIDL!{#[uuid(0xde8db6f8, 0xd101, 0x3a92, 0x8d, 0x1c, 0xe7, 0x2e, 0x5f, 0x10, 0xe9, 0x92)]
interface ICollection(ICollectionVtbl): IDispatch(IDispatchVtbl)  
{
    fn CopyTo(
        Array: *mut _Array,
        index: c_long,
    ) -> HRESULT,
    fn get_Count(
        pRetVal: *mut c_long ,
    ) -> HRESULT,
    fn get_SyncRoot(
        pRetVal: *mut VARIANT ,
    ) -> HRESULT,
      fn get_IsSynchronized(
        pRetVal: *mut VARIANT_BOOL ,
    ) -> HRESULT,
}}//inherits from IEnumerable