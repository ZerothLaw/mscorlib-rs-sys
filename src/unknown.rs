
use winapi::shared::guiddef::REFIID;
use winapi::shared::minwindef::{WORD,UINT};
use winapi::shared::ntdef::{LCID};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::{BSTR};
use winapi::shared::wtypesbase::LPOLESTR;

use winapi::um::oaidl::{DISPID, DISPPARAMS, EXCEPINFO, ITypeInfo, VARIANT};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

RIDL!{#[uuid(0x00020404, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IEnumVARIANT(IEnumVARIANTVtbl): IUnknown(IUnknownVtbl){
    //methods are unknown
}}

RIDL!{#[uuid(0x03973551, 0x57a1, 0x3900, 0xa2, 0xb5, 0x90, 0x83, 0xe3, 0xff, 0x29, 0x43)]
interface _Activator(_ActivatorVtbl) : IUnknown(IUnknownVtbl){
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

RIDL!{#[uuid(0x27fff232, 0xa7a8, 0x40dd, 0x8d, 0x4a, 0x73, 0x4a, 0xd5, 0x9f, 0xcd, 0x41)]
interface IAppDomainSetup(IAppDomainSetupVtbl) : IUnknown(IUnknownVtbl){
    fn get_ApplicationBase(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    
    fn put_ApplicationBase(
        pRetVal: BSTR,
    ) -> HRESULT,

    fn get_ApplicationName(
        pRetVal: *mut BSTR,
    ) -> HRESULT,

    fn put_ApplicationName(
        pRetVal: BSTR, 
    ) -> HRESULT,

    fn get_CachePath(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
      
    fn put_CachePath(
        pRetVal: BSTR, 
    ) -> HRESULT,

    fn get_ConfigurationFile(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
      
    fn put_ConfigurationFile(
        pRetVal: BSTR, 
    ) -> HRESULT,

    fn get_DynamicBase(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
      
    fn put_DynamicBase(
        pRetVal: BSTR, 
    ) -> HRESULT,

    fn get_LicenseFile(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
      
    fn put_LicenseFile(
        pRetVal: BSTR, 
    ) -> HRESULT,

    fn get_PrivateBinPath(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
      
    fn put_PrivateBinPath(
        pRetVal: BSTR, 
    ) -> HRESULT,

    fn get_PrivateBinPathProbe(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
      
    fn put_PrivateBinPathProbe(
        pRetVal: BSTR, 
    ) -> HRESULT,

    fn get_ShadowCopyDirectories(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
      
    fn put_ShadowCopyDirectories(
        pRetVal: BSTR, 
    ) -> HRESULT,

    fn get_ShadowCopyFiles(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
      
    fn put_ShadowCopyFiles(
        pRetVal: BSTR, 
    ) -> HRESULT,

}}

RIDL!{#[uuid(0x917b14d0, 0x2d9e, 0x38b8, 0x92, 0xa9, 0x38, 0x1a, 0xcf, 0x52, 0xf7, 0xc0)]
interface _Attribute(_AttributeVtbl) : IUnknown(IUnknownVtbl){
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

RIDL!{#[uuid(0xc460e2b4, 0xe199, 0x412a, 0x84, 0x56, 0x84, 0xdc, 0x3e, 0x48, 0x38, 0xc3)]
interface IObjectHandle(IObjectHandleVtbl) : IUnknown(IUnknownVtbl) {
  
    fn Unwrap(
        pRetVal: *mut VARIANT,
    ) -> HRESULT,

}}