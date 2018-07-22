use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::wtypes::BSTR;
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::shared::winerror::HRESULT;

RIDL!{#[uuid(0xf4205a87, 0x4d46, 0x303d, 0xb1, 0xd9, 0x5a, 0x99, 0xf7, 0xc9, 0x0d, 0x30)]
interface IIdentity(IIdentityVtbl) : IDispatch(IDispatchVtbl) {
    fn get_name(
        pRetVal: *mut BSTR, 
    ) -> HRESULT, 

    fn get_AuthenticationType(
        pRetVal: *mut BSTR, 
    ) -> HRESULT,

    fn get_IsAuthenticated(
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}