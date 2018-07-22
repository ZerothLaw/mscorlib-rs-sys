use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::{BSTR};
use winapi::um::oaidl::{SAFEARRAY, VARIANT};

RIDL!{#[uuid(0xe97aa6e5, 0x595e, 0x31c3, 0x82, 0xf0, 0x68, 0x8f, 0xb9, 0x19, 0x54, 0xc6)]
interface IResourceWriter(IResourceWriterVtbl) : IDispatch(IDispatchVtbl)
{
    fn AddResource(
        name: BSTR, 
        val: BSTR,
    ) -> HRESULT,
    fn AddResource_2(
        name: BSTR, 
        val: VARIANT,
    ) -> HRESULT,
    fn AddResource_3(
        name: BSTR, 
        val: *mut SAFEARRAY, //byte[]
    ) -> HRESULT,
    fn Close() -> HRESULT, 
    fn Generate() -> HRESULT,
}}