
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::shared::minwindef::UINT;
use winapi::shared::minwindef::WORD;
use winapi::shared::ntdef::LCID;
use winapi::shared::guiddef::REFIID;
use winapi::um::oaidl::EXCEPINFO;
use winapi::um::oaidl::VARIANT;
use winapi::um::oaidl::DISPPARAMS;
use winapi::um::oaidl::DISPID;
use winapi::shared::wtypesbase::LPOLESTR;
use winapi::um::oaidl::ITypeInfo;

RIDL!{#[uuid(0xc281c7f1, 0x4aa9, 0x3517, 0x96, 0x1a, 0x46, 0x3c, 0xfe, 0xd5, 0x7e, 0x75)]
interface _Thread(_ThreadVtbl) : IUnknown(IUnknownVtbl){
    fn GetTypeInfoCount(
        pctinfo: *mut UINT,
    ) -> HRESULT,
    fn GetTypeInfo(
        iTInfo: UINT,
        lcid: LCID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetIDsOfNames(
        riid: REFIID,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        lcid: LCID,
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke(
        dispIdMember: DISPID,
        riid: REFIID,
        lcid: LCID,
        wFlags: WORD,
        pDispParams: *mut DISPPARAMS,
        pVarResult: *mut VARIANT,
        pExcepInfo: *mut EXCEPINFO,
        puArgErr: *mut UINT,
    ) -> HRESULT,
}}