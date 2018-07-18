use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::BSTR;

use source::system::security::principal::iidentity::IIdentity;

RIDL!{#[uuid(0x4283ca6c, 0xd291, 0x3481, 0x83, 0xc9, 0x95, 0x54, 0x48, 0x1f, 0xe8, 0x88)]
interface IPrincipal(IPrincipalVtbl) : IDispatch(IDispatchVtbl) {
    fn get_Identity(
        pRetVal: *mut *mut IIdentity,
    ) -> HRESULT,

    fn IsInRole(
        role: BSTR, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}
