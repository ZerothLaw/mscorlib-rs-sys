
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

struct __declspec(uuid("bebb2505-8b54-3443-aead-142a16dd9cc7"))
_AssemblyBuilder : IUnknown
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

struct __declspec(uuid("ed3e4384-d7e2-3fa7-8ffd-8940d330519a"))
_ConstructorBuilder : IUnknown
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

struct __declspec(uuid("be9acce8-aaff-3b91-81ae-8211663f5cad"))
_CustomAttributeBuilder : IUnknown
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

struct __declspec(uuid("c7bd73de-9f85-3290-88ee-090b8bdfe2df"))
_EnumBuilder : IUnknown
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

struct __declspec(uuid("aadaba99-895d-3d65-9760-b1f12621fae8"))
_EventBuilder : IUnknown
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

struct __declspec(uuid("ce1a3bf5-975e-30cc-97c9-1ef70f8f3993"))
_FieldBuilder : IUnknown
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

struct __declspec(uuid("a4924b27-6e3b-37f7-9b83-a4501955e6a7"))
_ILGenerator : IUnknown
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

struct __declspec(uuid("4e6350d1-a08b-3dec-9a3e-c465f9aeec0c"))
_LocalBuilder : IUnknown
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

struct __declspec(uuid("007d8a14-fdf3-363e-9a0b-fec0618260a2"))
_MethodBuilder : IUnknown
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

struct __declspec(uuid("c2323c25-f57f-3880-8a4d-12ebea7a5852"))
_MethodRental : IUnknown
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

struct __declspec(uuid("d05ffa9a-04af-3519-8ee1-8d93ad73430b"))
_ModuleBuilder : IUnknown
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

struct __declspec(uuid("36329eba-f97a-3565-bc07-0ed5c6ef19fc"))
_ParameterBuilder : IUnknown
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

struct __declspec(uuid("15f9a479-9397-3a63-acbd-f51977fb0f02"))
_PropertyBuilder : IUnknown
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

struct __declspec(uuid("7d13dd37-5a04-393c-bbca-a5fea802893d"))
_SignatureHelper : IUnknown
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

struct __declspec(uuid("7e5678ee-48b3-3f83-b076-c58543498a58"))
_TypeBuilder : IUnknown
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


struct __declspec(uuid("f7102fa9-cabb-3a74-a6da-b4567ef1b079"))
_MemberInfo : IUnknown
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

struct __declspec(uuid("bca8b44d-aad6-3a86-8ab7-03349f4f2da2"))
_Type : IUnknown
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


struct __declspec(uuid("f1c3bf77-c3e4-11d3-88e7-00902754c43a"))
ITypeLibExporterNotifySink : IUnknown
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

struct __declspec(uuid("f1c3bf78-c3e4-11d3-88e7-00902754c43a"))
ITypeLibConverter : IUnknown
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

struct __declspec(uuid("6240837a-707f-3181-8e98-a36ae086766b"))
_MethodBase : IUnknown
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


struct __declspec(uuid("ffcc1b5d-ecb8-38dd-9b01-3dc8abc2aa5f"))
_MethodInfo : IUnknown
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


struct __declspec(uuid("9de59c64-d889-35a1-b897-587d74469e5b"))
_EventInfo : IUnknown
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

struct __declspec(uuid("e9a19478-9646-3679-9b10-8411ae1fd57d"))
_ConstructorInfo : IUnknown
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

struct __declspec(uuid("8a7c1442-a9fb-366b-80d8-4939ffa6dbe0"))
_FieldInfo : IUnknown
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

struct __declspec(uuid("f59ed4e4-e68f-3218-bd77-061aa82824bf"))
_PropertyInfo : IUnknown
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

