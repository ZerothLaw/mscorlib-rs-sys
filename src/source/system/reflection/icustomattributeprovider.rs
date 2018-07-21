use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::um::oaidl::SAFEARRAY;

use system::reflection::cominterfaces::_Type;


RIDL!{#[uuid(0xb9b91146, 0xd6c2, 0x3a62, 0x81, 0x59, 0xc2, 0xd1, 0x79, 0x4c, 0xde, 0xb0)]
interface ICustomAttributeProvider(ICustomAttributeProviderVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetCustomAttributes(
        attributeType: *mut _Type,
        inherit: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetCustomAttributes_2(
        inherit: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn IsDefined(
        attributeType: *mut _Type,
        inherit: VARIANT_BOOL,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}
