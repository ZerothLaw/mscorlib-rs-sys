use winapi::um::oaidl::{VARIANT};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};


use winapi::ctypes::c_long;
use winapi::shared::winerror::HRESULT;

RIDL!{#[uuid(0x5d573036, 0x3435, 0x3c5a, 0xae, 0xff, 0x2b, 0x81, 0x91, 0x08, 0x2c, 0x71)]
interface IHashCodeProvider(IHashCodeProviderVtbl) : IDispatch(IDispatchVtbl) {
    fn GetHashCode(
        obj: VARIANT, 
        pRetVal: *mut c_long,
    ) -> HRESULT,
}}