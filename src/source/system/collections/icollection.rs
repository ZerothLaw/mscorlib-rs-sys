//.Net Framework 4.7.2 Reference Source - mscorlib { system.collections.icollection.cs }


//RIDL!{#[uuid()]}
//"([\w\d]{8})-([\w\d]{4})-([\w\d]{4})-([\w\d]{2})([\w\d]{2})-([\w\d]{2})([\w\d]{2})([\w\d]{2})([\w\d]{2})([\w\d]{2})([\w\d]{2})"
//0x$1, 0x$2, 0x$3, 0x$4, 0x$5, 0x$6, 0x$7, 0x$8, 0x$9, 0x$10, 0x$11

use winapi::um::oaidl::{VARIANT};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};


use winapi::ctypes::c_long;
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::VARIANT_BOOL;


use dispatch::_Array;
RIDL!{#[uuid(0xde8db6f8, 0xd101, 0x3a92, 0x8d, 0x1c, 0xe7, 0x2e, 0x5f, 0x10, 0xe9, 0x92)]
interface ICollection(ICollectionVtbl): IDispatch(IDispatchVtbl)  
{
    fn CopyTo(
        Array: *mut _Array,
        index: c_long,
    ) -> HRESULT,
    fn get_Count(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn get_SyncRoot(
        pRetVal: VARIANT ,
    ) -> HRESULT,
      fn get_IsSynchronized(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
}}//inherits from IEnumerable