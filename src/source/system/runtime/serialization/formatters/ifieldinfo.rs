use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::SAFEARRAY;

RIDL!{#[uuid(0xcc18fd4d, 0xaa2d, 0x3ab4, 0x98, 0x48, 0x58, 0x4b, 0xba, 0xe4, 0xab, 0x44)]
interface IFieldInfo(IFieldInfoVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_FieldNames(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn put_FieldNames(
        pRetVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_FieldTypes(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn put_FieldTypes(
        pRetVal: *mut SAFEARRAY,
    ) -> HRESULT,
}}