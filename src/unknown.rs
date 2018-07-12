
use winapi::shared::guiddef::REFIID;
use winapi::shared::minwindef::{WORD,UINT};
use winapi::shared::ntdef::{LCID};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::{BSTR};
use winapi::shared::wtypesbase::LPOLESTR;

use winapi::um::oaidl::{DISPID, DISPPARAMS, EXCEPINFO, ITypeInfo, SAFEARRAY, VARIANT};
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

RIDL!{#[uuid(0xb42b6aac, 0x317e, 0x34d5, 0x9f, 0xa9, 0x09, 0x3b, 0xb4, 0x16, 0x0c, 0x50)]
interface _AssemblyName(_AssemblyNameVtbl) : IUnknown(IUnknownVtbl) {

    fn GetTypeInfoCount(
        pcTInfo: *mut UINT,
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

RIDL!{#[uuid(0x993634c4, 0xe47a, 0x32cc, 0xbe, 0x08, 0x85, 0xf5, 0x67, 0xdc, 0x27, 0xd6)]
interface _ParameterInfo(_ParameterInfoVtbl) : IUnknown(IUnknownVtbl) {
    fn GetTypeInfoCount(
        pcTInfo: *mut UINT,
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

RIDL!{#[uuid(0xd002e9ba, 0xd9e3, 0x3749, 0xb1, 0xd3, 0xd5, 0x65, 0xa0, 0x8b, 0x13, 0xe7)]
interface _Module(_ModuleVtbl) : IUnknown(IUnknownVtbl) {
    fn GetTypeInfoCount(
        pcTInfo: *mut UINT,
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

RIDL!{#[uuid(0xfa1f3615, 0xacb9, 0x486d, 0x9e, 0xac, 0x1b, 0xef, 0x87, 0xe3, 0x6b, 0x09)]
interface ITypeLibExporterNameProvider(ITypeLibExporterNameProviderVtbl) : IUnknown(IUnknownVtbl)
{
    fn GetNames(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xbebb2505, 0x8b54, 0x3443, 0xae, 0xad, 0x14, 0x2a, 0x16, 0xdd, 0x9c, 0xc7)]
interface _AssemblyBuilder(_AssemblyBuilderVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
};

RIDL!{#[uuid(0xed3e4384, 0xd7e2, 0x3fa7, 0x8f, 0xfd, 0x89, 0x40, 0xd3, 0x30, 0x51, 0x9a)]
interface _ConstructorBuilder(_ConstructorBuilderVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
};

RIDL!{#[uuid(0xbe9acce8, 0xaaff, 0x3b91, 0x81, 0xae, 0x82, 0x11, 0x66, 0x3f, 0x5c, 0xad)]
interface _CustomAttributeBuilder(_CustomAttributeBuilderVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
};

RIDL!{#[uuid(0xc7bd73de, 0x9f85, 0x3290, 0x88, 0xee, 0x09, 0x0b, 0x8b, 0xdf, 0xe2, 0xdf)]
interface _EnumBuilder(_EnumBuilderVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
};

RIDL!{#[uuid(0xaadaba99, 0x895d, 0x3d65, 0x97, 0x60, 0xb1, 0xf1, 0x26, 0x21, 0xfa, 0xe8)]
interface _EventBuilder(_EventBuilderVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
};

RIDL!{#[uuid(0xce1a3bf5, 0x975e, 0x30cc, 0x97, 0xc9, 0x1e, 0xf7, 0x0f, 0x8f, 0x39, 0x93)]
interface _FieldBuilder(_FieldBuilderVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
};

RIDL!{#[uuid(0xa4924b27, 0x6e3b, 0x37f7, 0x9b, 0x83, 0xa4, 0x50, 0x19, 0x55, 0xe6, 0xa7)]
interface _ILGenerator(_ILGeneratorVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
};

RIDL!{#[uuid(0x4e6350d1, 0xa08b, 0x3dec, 0x9a, 0x3e, 0xc4, 0x65, 0xf9, 0xae, 0xec, 0x0c)]
interface _LocalBuilder(_LocalBuilderVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
};

RIDL!{#[uuid(0x007d8a14, 0xfdf3, 0x363e, 0x9a, 0x0b, 0xfe, 0xc0, 0x61, 0x82, 0x60, 0xa2)]
interface _MethodBuilder(_MethodBuilderVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
};

RIDL!{#[uuid(0xc2323c25, 0xf57f, 0x3880, 0x8a, 0x4d, 0x12, 0xeb, 0xea, 0x7a, 0x58, 0x52)]
interface _MethodRental(_MethodRentalVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
};

RIDL!{#[uuid(0xd05ffa9a, 0x04af, 0x3519, 0x8e, 0xe1, 0x8d, 0x93, 0xad, 0x73, 0x43, 0x0b)]
interface _ModuleBuilder(_ModuleBuilderVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
};

RIDL!{#[uuid(0x36329eba, 0xf97a, 0x3565, 0xbc, 0x07, 0x0e, 0xd5, 0xc6, 0xef, 0x19, 0xfc)]
interface _ParameterBuilder(_ParameterBuilderVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
};

RIDL!{#[uuid(0x15f9a479, 0x9397, 0x3a63, 0xac, 0xbd, 0xf5, 0x19, 0x77, 0xfb, 0x0f, 0x02)]
interface _PropertyBuilder(_PropertyBuilderVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
};

RIDL!{#[uuid(0x7d13dd37, 0x5a04, 0x393c, 0xbb, 0xca, 0xa5, 0xfe, 0xa8, 0x02, 0x89, 0x3d)]
interface _SignatureHelper(_SignatureHelperVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
};

RIDL!{#[uuid(0x7e5678ee, 0x48b3, 0x3f83, 0xb0, 0x76, 0xc5, 0x85, 0x43, 0x49, 0x8a, 0x58)]
interface _TypeBuilder(_TypeBuilderVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
};


RIDL!{#[uuid(0xf7102fa9, 0xcabb, 0x3a74, 0xa6, 0xda, 0xb4, 0x56, 0x7e, 0xf1, 0xb0, 0x79)]
interface _MemberInfo(_MemberInfoVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
      virtual HRESULT __stdcall get_ToString (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall Equals (
        /*[in]*/ VARIANT other,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetHashCode (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall GetType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_MemberType (
        /*[out,retval]*/ enum MemberTypes * pRetVal ) = 0;
      virtual HRESULT __stdcall get_name (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_DeclaringType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ReflectedType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetCustomAttributes (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetCustomAttributes_2 (
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall IsDefined (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
};

RIDL!{#[uuid(0xbca8b44d, 0xaad6, 0x3a86, 0x8a, 0xb7, 0x03, 0x34, 0x9f, 0x4f, 0x2d, 0xa2)]
interface _Type(_TypeVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
      virtual HRESULT __stdcall get_ToString (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall Equals (
        /*[in]*/ VARIANT other,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetHashCode (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall GetType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_MemberType (
        /*[out,retval]*/ enum MemberTypes * pRetVal ) = 0;
      virtual HRESULT __stdcall get_name (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_DeclaringType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ReflectedType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetCustomAttributes (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetCustomAttributes_2 (
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall IsDefined (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Guid (
        /*[out,retval]*/ GUID * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Module (
        /*[out,retval]*/ struct _Module * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Assembly (
        /*[out,retval]*/ struct _Assembly * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_TypeHandle (
        /*[out,retval]*/ struct RuntimeTypeHandle * pRetVal ) = 0;
      virtual HRESULT __stdcall get_FullName (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Namespace (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_AssemblyQualifiedName (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall GetArrayRank (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall get_BaseType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetConstructors (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetInterface (
        /*[in]*/ BSTR name,
        /*[in]*/ VARIANT_BOOL ignoreCase,
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetInterfaces (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall FindInterfaces (
        /*[in]*/ struct _TypeFilter * filter,
        /*[in]*/ VARIANT filterCriteria,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetEvent (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ struct _EventInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetEvents (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetEvents_2 (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetNestedTypes (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetNestedType (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMember (
        /*[in]*/ BSTR name,
        /*[in]*/ enum MemberTypes Type,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetDefaultMembers (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall FindMembers (
        /*[in]*/ enum MemberTypes MemberType,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[in]*/ struct _MemberFilter * filter,
        /*[in]*/ VARIANT filterCriteria,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetElementType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall IsSubclassOf (
        /*[in]*/ struct _Type * c,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall IsInstanceOfType (
        /*[in]*/ VARIANT o,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall IsAssignableFrom (
        /*[in]*/ struct _Type * c,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetInterfaceMap (
        /*[in]*/ struct _Type * interfaceType,
        /*[out,retval]*/ struct InterfaceMapping * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethod (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ SAFEARRAY * types,
        /*[in]*/ SAFEARRAY * modifiers,
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethod_2 (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethods (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetField (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ struct _FieldInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetFields (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetProperty (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ struct _PropertyInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetProperty_2 (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ struct _Type * returnType,
        /*[in]*/ SAFEARRAY * types,
        /*[in]*/ SAFEARRAY * modifiers,
        /*[out,retval]*/ struct _PropertyInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetProperties (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMember_2 (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMembers (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall InvokeMember (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags invokeAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ VARIANT Target,
        /*[in]*/ SAFEARRAY * args,
        /*[in]*/ SAFEARRAY * modifiers,
        /*[in]*/ struct _CultureInfo * culture,
        /*[in]*/ SAFEARRAY * namedParameters,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall get_UnderlyingSystemType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall InvokeMember_2 (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags invokeAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ VARIANT Target,
        /*[in]*/ SAFEARRAY * args,
        /*[in]*/ struct _CultureInfo * culture,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall InvokeMember_3 (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags invokeAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ VARIANT Target,
        /*[in]*/ SAFEARRAY * args,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall GetConstructor (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ enum CallingConventions callConvention,
        /*[in]*/ SAFEARRAY * types,
        /*[in]*/ SAFEARRAY * modifiers,
        /*[out,retval]*/ struct _ConstructorInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetConstructor_2 (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ SAFEARRAY * types,
        /*[in]*/ SAFEARRAY * modifiers,
        /*[out,retval]*/ struct _ConstructorInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetConstructor_3 (
        /*[in]*/ SAFEARRAY * types,
        /*[out,retval]*/ struct _ConstructorInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetConstructors_2 (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_TypeInitializer (
        /*[out,retval]*/ struct _ConstructorInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethod_3 (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ enum CallingConventions callConvention,
        /*[in]*/ SAFEARRAY * types,
        /*[in]*/ SAFEARRAY * modifiers,
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethod_4 (
        /*[in]*/ BSTR name,
        /*[in]*/ SAFEARRAY * types,
        /*[in]*/ SAFEARRAY * modifiers,
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethod_5 (
        /*[in]*/ BSTR name,
        /*[in]*/ SAFEARRAY * types,
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethod_6 (
        /*[in]*/ BSTR name,
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethods_2 (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetField_2 (
        /*[in]*/ BSTR name,
        /*[out,retval]*/ struct _FieldInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetFields_2 (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetInterface_2 (
        /*[in]*/ BSTR name,
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetEvent_2 (
        /*[in]*/ BSTR name,
        /*[out,retval]*/ struct _EventInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetProperty_3 (
        /*[in]*/ BSTR name,
        /*[in]*/ struct _Type * returnType,
        /*[in]*/ SAFEARRAY * types,
        /*[in]*/ SAFEARRAY * modifiers,
        /*[out,retval]*/ struct _PropertyInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetProperty_4 (
        /*[in]*/ BSTR name,
        /*[in]*/ struct _Type * returnType,
        /*[in]*/ SAFEARRAY * types,
        /*[out,retval]*/ struct _PropertyInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetProperty_5 (
        /*[in]*/ BSTR name,
        /*[in]*/ SAFEARRAY * types,
        /*[out,retval]*/ struct _PropertyInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetProperty_6 (
        /*[in]*/ BSTR name,
        /*[in]*/ struct _Type * returnType,
        /*[out,retval]*/ struct _PropertyInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetProperty_7 (
        /*[in]*/ BSTR name,
        /*[out,retval]*/ struct _PropertyInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetProperties_2 (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetNestedTypes_2 (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetNestedType_2 (
        /*[in]*/ BSTR name,
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMember_3 (
        /*[in]*/ BSTR name,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMembers_2 (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Attributes (
        /*[out,retval]*/ enum TypeAttributes * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsNotPublic (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsPublic (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsNestedPublic (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsNestedPrivate (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsNestedFamily (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsNestedAssembly (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsNestedFamANDAssem (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsNestedFamORAssem (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsAutoLayout (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsLayoutSequential (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsExplicitLayout (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsClass (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsInterface (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsValueType (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsAbstract (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsSealed (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsEnum (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsSpecialName (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsImport (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsSerializable (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsAnsiClass (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsUnicodeClass (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsAutoClass (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsArray (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsByRef (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsPointer (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsPrimitive (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsCOMObject (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_HasElementType (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsContextful (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsMarshalByRef (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall Equals_2 (
        /*[in]*/ struct _Type * o,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
};


RIDL!{#[uuid(0xf1c3bf77, 0xc3e4, 0x11d3, 0x88, 0xe7, 0x00, 0x90, 0x27, 0x54, 0xc4, 0x3a)]
interface ITypeLibExporterNotifySink(ITypeLibExporterNotifySinkVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall InteropServices_ReportEvent (
        /*[in]*/ enum ExporterEventKind eventKind,
        /*[in]*/ long eventCode,
        /*[in]*/ BSTR eventMsg ) = 0;
      virtual HRESULT __stdcall ResolveRef (
        /*[in]*/ struct _Assembly * Assembly,
        /*[out,retval]*/ IUnknown * * pRetVal ) = 0;
};

RIDL!{#[uuid(0xf1c3bf78, 0xc3e4, 0x11d3, 0x88, 0xe7, 0x00, 0x90, 0x27, 0x54, 0xc4, 0x3a)]
interface ITypeLibConverter(ITypeLibConverterVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall ConvertTypeLibToAssembly (
        /*[in]*/ IUnknown * typeLib,
        /*[in]*/ BSTR asmFileName,
        /*[in]*/ enum TypeLibImporterFlags flags,
        /*[in]*/ struct ITypeLibImporterNotifySink * notifySink,
        /*[in]*/ SAFEARRAY * publicKey,
        /*[in]*/ struct _StrongNameKeyPair * keyPair,
        /*[in]*/ BSTR asmNamespace,
        /*[in]*/ struct _Version * asmVersion,
        /*[out,retval]*/ struct _AssemblyBuilder * * pRetVal ) = 0;
      virtual HRESULT __stdcall ConvertAssemblyToTypeLib (
        /*[in]*/ struct _Assembly * Assembly,
        /*[in]*/ BSTR typeLibName,
        /*[in]*/ enum TypeLibExporterFlags flags,
        /*[in]*/ struct ITypeLibExporterNotifySink * notifySink,
        /*[out,retval]*/ IUnknown * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetPrimaryInteropAssembly (
        /*[in]*/ GUID G,
        /*[in]*/ long major,
        /*[in]*/ long minor,
        /*[in]*/ long lcid,
        /*[out]*/ BSTR * asmName,
        /*[out]*/ BSTR * asmCodeBase,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall ConvertTypeLibToAssembly_2 (
        /*[in]*/ IUnknown * typeLib,
        /*[in]*/ BSTR asmFileName,
        /*[in]*/ long flags,
        /*[in]*/ struct ITypeLibImporterNotifySink * notifySink,
        /*[in]*/ SAFEARRAY * publicKey,
        /*[in]*/ struct _StrongNameKeyPair * keyPair,
        /*[in]*/ VARIANT_BOOL unsafeInterfaces,
        /*[out,retval]*/ struct _AssemblyBuilder * * pRetVal ) = 0;
};

RIDL!{#[uuid(0x6240837a, 0x707f, 0x3181, 0x8e, 0x98, 0xa3, 0x6a, 0xe0, 0x86, 0x76, 0x6b)]
interface _MethodBase(_MethodBaseVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
      virtual HRESULT __stdcall get_ToString (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall Equals (
        /*[in]*/ VARIANT other,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetHashCode (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall GetType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_MemberType (
        /*[out,retval]*/ enum MemberTypes * pRetVal ) = 0;
      virtual HRESULT __stdcall get_name (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_DeclaringType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ReflectedType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetCustomAttributes (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetCustomAttributes_2 (
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall IsDefined (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetParameters (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethodImplementationFlags (
        /*[out,retval]*/ enum MethodImplAttributes * pRetVal ) = 0;
      virtual HRESULT __stdcall get_MethodHandle (
        /*[out,retval]*/ struct RuntimeMethodHandle * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Attributes (
        /*[out,retval]*/ enum MethodAttributes * pRetVal ) = 0;
      virtual HRESULT __stdcall get_CallingConvention (
        /*[out,retval]*/ enum CallingConventions * pRetVal ) = 0;
      virtual HRESULT __stdcall Invoke_2 (
        /*[in]*/ VARIANT obj,
        /*[in]*/ enum BindingFlags invokeAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ SAFEARRAY * parameters,
        /*[in]*/ struct _CultureInfo * culture,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsPublic (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsPrivate (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFamily (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsAssembly (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFamilyAndAssembly (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFamilyOrAssembly (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsStatic (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFinal (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsVirtual (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsHideBySig (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsAbstract (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsSpecialName (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsConstructor (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall Invoke_3 (
        /*[in]*/ VARIANT obj,
        /*[in]*/ SAFEARRAY * parameters,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
};


RIDL!{#[uuid(0xffcc1b5d, 0xecb8, 0x38dd, 0x9b, 0x01, 0x3d, 0xc8, 0xab, 0xc2, 0xaa, 0x5f)]
interface _MethodInfo(_MethodInfoVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
      virtual HRESULT __stdcall get_ToString (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall Equals (
        /*[in]*/ VARIANT other,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetHashCode (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall GetType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_MemberType (
        /*[out,retval]*/ enum MemberTypes * pRetVal ) = 0;
      virtual HRESULT __stdcall get_name (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_DeclaringType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ReflectedType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetCustomAttributes (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetCustomAttributes_2 (
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall IsDefined (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetParameters (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethodImplementationFlags (
        /*[out,retval]*/ enum MethodImplAttributes * pRetVal ) = 0;
      virtual HRESULT __stdcall get_MethodHandle (
        /*[out,retval]*/ struct RuntimeMethodHandle * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Attributes (
        /*[out,retval]*/ enum MethodAttributes * pRetVal ) = 0;
      virtual HRESULT __stdcall get_CallingConvention (
        /*[out,retval]*/ enum CallingConventions * pRetVal ) = 0;
      virtual HRESULT __stdcall Invoke_2 (
        /*[in]*/ VARIANT obj,
        /*[in]*/ enum BindingFlags invokeAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ SAFEARRAY * parameters,
        /*[in]*/ struct _CultureInfo * culture,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsPublic (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsPrivate (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFamily (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsAssembly (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFamilyAndAssembly (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFamilyOrAssembly (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsStatic (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFinal (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsVirtual (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsHideBySig (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsAbstract (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsSpecialName (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsConstructor (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall Invoke_3 (
        /*[in]*/ VARIANT obj,
        /*[in]*/ SAFEARRAY * parameters,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall get_returnType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ReturnTypeCustomAttributes (
        /*[out,retval]*/ struct ICustomAttributeProvider * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetBaseDefinition (
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
};


RIDL!{#[uuid(0x9de59c64, 0xd889, 0x35a1, 0xb8, 0x97, 0x58, 0x7d, 0x74, 0x46, 0x9e, 0x5b)]
interface _EventInfo(_EventInfoVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
      virtual HRESULT __stdcall get_ToString (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall Equals (
        /*[in]*/ VARIANT other,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetHashCode (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall GetType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_MemberType (
        /*[out,retval]*/ enum MemberTypes * pRetVal ) = 0;
      virtual HRESULT __stdcall get_name (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_DeclaringType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ReflectedType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetCustomAttributes (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetCustomAttributes_2 (
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall IsDefined (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetAddMethod (
        /*[in]*/ VARIANT_BOOL nonPublic,
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetRemoveMethod (
        /*[in]*/ VARIANT_BOOL nonPublic,
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetRaiseMethod (
        /*[in]*/ VARIANT_BOOL nonPublic,
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Attributes (
        /*[out,retval]*/ enum EventAttributes * pRetVal ) = 0;
      virtual HRESULT __stdcall GetAddMethod_2 (
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetRemoveMethod_2 (
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetRaiseMethod_2 (
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall AddEventHandler (
        /*[in]*/ VARIANT Target,
        /*[in]*/ struct _Delegate * handler ) = 0;
      virtual HRESULT __stdcall RemoveEventHandler (
        /*[in]*/ VARIANT Target,
        /*[in]*/ struct _Delegate * handler ) = 0;
      virtual HRESULT __stdcall get_EventHandlerType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsSpecialName (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsMulticast (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
};

RIDL!{#[uuid(0xe9a19478, 0x9646, 0x3679, 0x9b, 0x10, 0x84, 0x11, 0xae, 0x1f, 0xd5, 0x7d)]
interface _ConstructorInfo(_ConstructorInfoVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
      virtual HRESULT __stdcall get_ToString (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall Equals (
        /*[in]*/ VARIANT other,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetHashCode (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall GetType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_MemberType (
        /*[out,retval]*/ enum MemberTypes * pRetVal ) = 0;
      virtual HRESULT __stdcall get_name (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_DeclaringType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ReflectedType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetCustomAttributes (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetCustomAttributes_2 (
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall IsDefined (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetParameters (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethodImplementationFlags (
        /*[out,retval]*/ enum MethodImplAttributes * pRetVal ) = 0;
      virtual HRESULT __stdcall get_MethodHandle (
        /*[out,retval]*/ struct RuntimeMethodHandle * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Attributes (
        /*[out,retval]*/ enum MethodAttributes * pRetVal ) = 0;
      virtual HRESULT __stdcall get_CallingConvention (
        /*[out,retval]*/ enum CallingConventions * pRetVal ) = 0;
      virtual HRESULT __stdcall Invoke_2 (
        /*[in]*/ VARIANT obj,
        /*[in]*/ enum BindingFlags invokeAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ SAFEARRAY * parameters,
        /*[in]*/ struct _CultureInfo * culture,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsPublic (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsPrivate (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFamily (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsAssembly (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFamilyAndAssembly (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFamilyOrAssembly (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsStatic (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFinal (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsVirtual (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsHideBySig (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsAbstract (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsSpecialName (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsConstructor (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall Invoke_3 (
        /*[in]*/ VARIANT obj,
        /*[in]*/ SAFEARRAY * parameters,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall Invoke_4 (
        /*[in]*/ enum BindingFlags invokeAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ SAFEARRAY * parameters,
        /*[in]*/ struct _CultureInfo * culture,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall Invoke_5 (
        /*[in]*/ SAFEARRAY * parameters,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
};

RIDL!{#[uuid(0x8a7c1442, 0xa9fb, 0x366b, 0x80, 0xd8, 0x49, 0x39, 0xff, 0xa6, 0xdb, 0xe0)]
interface _FieldInfo(_FieldInfoVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
      virtual HRESULT __stdcall get_ToString (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall Equals (
        /*[in]*/ VARIANT other,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetHashCode (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall GetType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_MemberType (
        /*[out,retval]*/ enum MemberTypes * pRetVal ) = 0;
      virtual HRESULT __stdcall get_name (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_DeclaringType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ReflectedType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetCustomAttributes (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetCustomAttributes_2 (
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall IsDefined (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_FieldType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetValue (
        /*[in]*/ VARIANT obj,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall GetValueDirect (
        /*[in]*/ VARIANT obj,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall SetValue (
        /*[in]*/ VARIANT obj,
        /*[in]*/ VARIANT val,
        /*[in]*/ enum BindingFlags invokeAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ struct _CultureInfo * culture ) = 0;
      virtual HRESULT __stdcall SetValueDirect (
        /*[in]*/ VARIANT obj,
        /*[in]*/ VARIANT val ) = 0;
      virtual HRESULT __stdcall get_FieldHandle (
        /*[out,retval]*/ struct RuntimeFieldHandle * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Attributes (
        /*[out,retval]*/ enum FieldAttributes * pRetVal ) = 0;
      virtual HRESULT __stdcall SetValue_2 (
        /*[in]*/ VARIANT obj,
        /*[in]*/ VARIANT val ) = 0;
      virtual HRESULT __stdcall get_IsPublic (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsPrivate (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFamily (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsAssembly (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFamilyAndAssembly (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFamilyOrAssembly (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsStatic (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsInitOnly (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsLiteral (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsNotSerialized (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsSpecialName (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsPinvokeImpl (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
};

RIDL!{#[uuid(0xf59ed4e4, 0xe68f, 0x3218, 0xbd, 0x77, 0x06, 0x1a, 0xa8, 0x28, 0x24, 0xbf)]
interface _PropertyInfo(_PropertyInfoVtbl): IUnknown(IUnknownVtbl)   
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
      virtual HRESULT __stdcall get_ToString (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall Equals (
        /*[in]*/ VARIANT other,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetHashCode (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall GetType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_MemberType (
        /*[out,retval]*/ enum MemberTypes * pRetVal ) = 0;
      virtual HRESULT __stdcall get_name (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_DeclaringType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ReflectedType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetCustomAttributes (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetCustomAttributes_2 (
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall IsDefined (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_PropertyType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetValue (
        /*[in]*/ VARIANT obj,
        /*[in]*/ SAFEARRAY * index,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall GetValue_2 (
        /*[in]*/ VARIANT obj,
        /*[in]*/ enum BindingFlags invokeAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ SAFEARRAY * index,
        /*[in]*/ struct _CultureInfo * culture,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall SetValue (
        /*[in]*/ VARIANT obj,
        /*[in]*/ VARIANT val,
        /*[in]*/ SAFEARRAY * index ) = 0;
      virtual HRESULT __stdcall SetValue_2 (
        /*[in]*/ VARIANT obj,
        /*[in]*/ VARIANT val,
        /*[in]*/ enum BindingFlags invokeAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ SAFEARRAY * index,
        /*[in]*/ struct _CultureInfo * culture ) = 0;
      virtual HRESULT __stdcall GetAccessors (
        /*[in]*/ VARIANT_BOOL nonPublic,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetGetMethod (
        /*[in]*/ VARIANT_BOOL nonPublic,
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetSetMethod (
        /*[in]*/ VARIANT_BOOL nonPublic,
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetIndexParameters (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Attributes (
        /*[out,retval]*/ enum PropertyAttributes * pRetVal ) = 0;
      virtual HRESULT __stdcall get_CanRead (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_CanWrite (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetAccessors_2 (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetGetMethod_2 (
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetSetMethod_2 (
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsSpecialName (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
};

