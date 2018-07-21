use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::um::oaidl::VARIANT;
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::BSTR;
use winapi::shared::wtypes::VARIANT_BOOL;

use system::security::policy::evidence::_Evidence;

RIDL!{#[uuid(0x6844eff4, 0x4f86, 0x3ca1, 0xa1, 0xea, 0xaa, 0xf5, 0x83, 0xa6, 0x39, 0x5e)]
interface IMembershipCondition(IMembershipConditionVtbl): IDispatch(IDispatchVtbl)  
{
    fn Check(
		Evidence: *mut  _Evidence ,
		pRetVal: *mut VARIANT_BOOL ,
	) -> HRESULT,
    fn Copy(
		pRetVal: *mut *mut  IMembershipCondition ,
	) -> HRESULT,
    fn get_ToString(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn Equals(
        obj: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}