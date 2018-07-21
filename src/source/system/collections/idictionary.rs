
use winapi::um::oaidl::{VARIANT};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::VARIANT_BOOL;

use system::collections::icollection::ICollection;
use system::collections::idictionaryenumerator::IDictionaryEnumerator;

RIDL!{#[uuid(0x6a6841df, 0x3287, 0x3d87, 0x80, 0x60, 0xce, 0x0b, 0x4c, 0x77, 0xd2, 0xa1)]
interface IDictionary(IDictionaryVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_Item(
        key: VARIANT,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn putref_Item(
        key: VARIANT,
        pRetVal: VARIANT,
    ) -> HRESULT,
    fn get_Keys(
		pRetVal: *mut *mut  ICollection ,
	) -> HRESULT,
    fn get_Values(
		pRetVal: *mut *mut  ICollection ,
	) -> HRESULT,
    fn Contains(
        key: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Add(
        key: VARIANT,
        val: VARIANT,
    ) -> HRESULT,
    fn Clear() -> HRESULT,
    fn get_IsReadOnly(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsFixedSize(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn GetEnumerator(
		pRetVal: *mut *mut  IDictionaryEnumerator ,
	) -> HRESULT,
    fn Remove(
        key: VARIANT,
    ) -> HRESULT,
}} //inherits ICollection