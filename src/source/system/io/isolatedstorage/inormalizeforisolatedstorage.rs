use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::ntdef::{HRESULT};
use winapi::um::oaidl::{VARIANT};

RIDL!{#[uuid(0xf5006531, 0xd4d7, 0x319e, 0x9e, 0xda, 0x9b, 0x4b, 0x65, 0xad, 0x8d, 0x4f)]
interface INormalizeForIsolatedStorage(INormalizeForIsolatedStorageVtbl): IDispatch(IDispatchVtbl)  
{
    fn Normalize(
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
}}