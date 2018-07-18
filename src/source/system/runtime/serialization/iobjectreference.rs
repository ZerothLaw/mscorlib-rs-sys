use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::um::oaidl::VARIANT;
use winapi::shared::winerror::HRESULT;


use structs::StreamingContext;

RIDL!{#[uuid(0x6e70ed5f, 0x0439, 0x38ce, 0x83, 0xbb, 0x86, 0x0f, 0x14, 0x21, 0xf2, 0x9f)]
interface IObjectReference(IObjectReferenceVtbl) : IDispatch(IDispatchVtbl) {
    fn GetRealObject(
        Context: StreamingContext, 
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
}}
