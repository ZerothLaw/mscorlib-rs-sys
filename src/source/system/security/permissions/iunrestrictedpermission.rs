use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::shared::winerror::HRESULT;

RIDL!{#[uuid(0x0f1284e6, 0x4399, 0x3963, 0x8d, 0xdd, 0xa6, 0xa4, 0x90, 0x4f, 0x66, 0xc8)]
interface IUnrestrictedPermission(IUnrestrictedPermissionVtbl): IDispatch(IDispatchVtbl)  
{
    fn IsUnrestricted(
        pRetVal: VARIANT_BOOL,
    ) -> HRESULT,
}}