#![allow(dead_code, non_snake_case)]
use winapi::ctypes::{c_long,};

use winapi::shared::guiddef::{GUID, REFIID};
use winapi::shared::minwindef::{WORD,UINT};
use winapi::shared::ntdef::{LCID};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};
use winapi::shared::wtypesbase::LPOLESTR;

use winapi::um::oaidl::{DISPID, DISPPARAMS, EXCEPINFO, ITypeInfo, SAFEARRAY, VARIANT};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use dispatch::*;

use system::_Delegate;
use system::reflection::_Assembly;
use system::reflection::_Binder;
use system::reflection::ICustomAttributeProvider;
use system::reflection::BindingFlags;
use system::reflection::CallingConventions;
use system::reflection::_Module;
use system::reflection::EventAttributes;
use system::reflection::FieldAttributes;
use system::reflection::MemberTypes;
use system::reflection::MethodAttributes;
use system::reflection::MethodImplAttributes;
use system::reflection::PropertyAttributes;
use system::reflection::TypeAttributes;
use system::runtime::interopservices::ExporterEventKind;
use system::runtime::interopservices::ImporterEventKind;
use system::runtime::interopservices::TypeLibExporterFlags;
use system::runtime::interopservices::TypeLibImporterFlags;

use system::reflection::InterfaceMapping;
use system::{RuntimeMethodHandle, RuntimeFieldHandle};
use system::RuntimeTypeHandle;

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

RIDL!{#[uuid(0xed3e4384, 0xd7e2, 0x3fa7, 0x8f, 0xfd, 0x89, 0x40, 0xd3, 0x30, 0x51, 0x9a)]
interface _ConstructorBuilder(_ConstructorBuilderVtbl): IUnknown(IUnknownVtbl)   
{
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

RIDL!{#[uuid(0xbe9acce8, 0xaaff, 0x3b91, 0x81, 0xae, 0x82, 0x11, 0x66, 0x3f, 0x5c, 0xad)]
interface _CustomAttributeBuilder(_CustomAttributeBuilderVtbl): IUnknown(IUnknownVtbl)   
{
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

RIDL!{#[uuid(0xc7bd73de, 0x9f85, 0x3290, 0x88, 0xee, 0x09, 0x0b, 0x8b, 0xdf, 0xe2, 0xdf)]
interface _EnumBuilder(_EnumBuilderVtbl): IUnknown(IUnknownVtbl)   
{
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

RIDL!{#[uuid(0xaadaba99, 0x895d, 0x3d65, 0x97, 0x60, 0xb1, 0xf1, 0x26, 0x21, 0xfa, 0xe8)]
interface _EventBuilder(_EventBuilderVtbl): IUnknown(IUnknownVtbl)   
{
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

RIDL!{#[uuid(0xce1a3bf5, 0x975e, 0x30cc, 0x97, 0xc9, 0x1e, 0xf7, 0x0f, 0x8f, 0x39, 0x93)]
interface _FieldBuilder(_FieldBuilderVtbl): IUnknown(IUnknownVtbl)   
{
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

RIDL!{#[uuid(0xa4924b27, 0x6e3b, 0x37f7, 0x9b, 0x83, 0xa4, 0x50, 0x19, 0x55, 0xe6, 0xa7)]
interface _ILGenerator(_ILGeneratorVtbl): IUnknown(IUnknownVtbl)   
{
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

RIDL!{#[uuid(0x4e6350d1, 0xa08b, 0x3dec, 0x9a, 0x3e, 0xc4, 0x65, 0xf9, 0xae, 0xec, 0x0c)]
interface _LocalBuilder(_LocalBuilderVtbl): IUnknown(IUnknownVtbl)   
{
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

RIDL!{#[uuid(0x007d8a14, 0xfdf3, 0x363e, 0x9a, 0x0b, 0xfe, 0xc0, 0x61, 0x82, 0x60, 0xa2)]
interface _MethodBuilder(_MethodBuilderVtbl): IUnknown(IUnknownVtbl)   
{
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

RIDL!{#[uuid(0xc2323c25, 0xf57f, 0x3880, 0x8a, 0x4d, 0x12, 0xeb, 0xea, 0x7a, 0x58, 0x52)]
interface _MethodRental(_MethodRentalVtbl): IUnknown(IUnknownVtbl)   
{
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

RIDL!{#[uuid(0xd05ffa9a, 0x04af, 0x3519, 0x8e, 0xe1, 0x8d, 0x93, 0xad, 0x73, 0x43, 0x0b)]
interface _ModuleBuilder(_ModuleBuilderVtbl): IUnknown(IUnknownVtbl)   
{
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

RIDL!{#[uuid(0x36329eba, 0xf97a, 0x3565, 0xbc, 0x07, 0x0e, 0xd5, 0xc6, 0xef, 0x19, 0xfc)]
interface _ParameterBuilder(_ParameterBuilderVtbl): IUnknown(IUnknownVtbl)   
{
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

RIDL!{#[uuid(0x15f9a479, 0x9397, 0x3a63, 0xac, 0xbd, 0xf5, 0x19, 0x77, 0xfb, 0x0f, 0x02)]
interface _PropertyBuilder(_PropertyBuilderVtbl): IUnknown(IUnknownVtbl)   
{
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

RIDL!{#[uuid(0x7d13dd37, 0x5a04, 0x393c, 0xbb, 0xca, 0xa5, 0xfe, 0xa8, 0x02, 0x89, 0x3d)]
interface _SignatureHelper(_SignatureHelperVtbl): IUnknown(IUnknownVtbl)   
{
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

RIDL!{#[uuid(0x7e5678ee, 0x48b3, 0x3f83, 0xb0, 0x76, 0xc5, 0x85, 0x43, 0x49, 0x8a, 0x58)]
interface _TypeBuilder(_TypeBuilderVtbl): IUnknown(IUnknownVtbl)   
{
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


RIDL!{#[uuid(0xf7102fa9, 0xcabb, 0x3a74, 0xa6, 0xda, 0xb4, 0x56, 0x7e, 0xf1, 0xb0, 0x79)]
interface _MemberInfo(_MemberInfoVtbl): IUnknown(IUnknownVtbl)   
{
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
    fn get_ToString(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn Equals(
        other: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetHashCode(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn GetType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_MemberType(
        pRetVal: MemberTypes ,
    ) -> HRESULT,
    fn get_name(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_DeclaringType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_ReflectedType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn GetCustomAttributes(
        attributeType: *mut _Type, 
        inherit: VARIANT_BOOL, 
        pRetVal: *mut *mut SAFEARRAY,
    ) ->HRESULT,
    fn GetCustomAttributes_2(
        inherit: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn IsDefined(
        attributeType: *mut _Type, 
        inherit: VARIANT_BOOL, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xbca8b44d, 0xaad6, 0x3a86, 0x8a, 0xb7, 0x03, 0x34, 0x9f, 0x4f, 0x2d, 0xa2)]
interface _Type(_TypeVtbl): IUnknown(IUnknownVtbl)   
{
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
    fn get_ToString(
        pRetVal: *mut BSTR ,
    ) -> HRESULT,
    fn Equals(
        other: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetHashCode(
        pRetVal: *mut  c_long ,
    ) -> HRESULT,
    fn GetType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_MemberType(
        pRetVal: MemberTypes ,
    ) -> HRESULT,
    fn get_name(
        pRetVal: *mut BSTR ,
    ) -> HRESULT,
    fn get_DeclaringType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_ReflectedType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn GetCustomAttributes(
        attributeType: *mut _Type, 
        inherit: VARIANT_BOOL, 
        pRetVal: *mut *mut SAFEARRAY,
    ) ->HRESULT,
    fn GetCustomAttributes_2(
        inherit: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn IsDefined(
        attributeType: *mut _Type, 
        inherit: VARIANT_BOOL, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Guid(
        pRetVal: *mut GUID ,
    ) -> HRESULT,
    fn get_Module(
        pRetVal: *mut *mut _Module ,
    ) -> HRESULT,
    fn get_Assembly(
        pRetVal: *mut *mut _Assembly ,
    ) -> HRESULT,
    fn get_TypeHandle(
        pRetVal: RuntimeTypeHandle ,
    ) -> HRESULT,
    fn get_FullName(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Namespace(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_AssemblyQualifiedName(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn GetArrayRank(
        pRetVal: *mut c_long ,
    ) -> HRESULT,
    fn get_BaseType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn GetConstructors(
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetInterface(
        name: BSTR, 
        ignoreCase: VARIANT_BOOL,
        pRetVal: *mut *mut _Type,
    ) -> HRESULT,
    fn GetInterfaces(
        pRetVal: *mut *mut SAFEARRAY ,
    ) -> HRESULT,
    fn FindInterfaces(
        filter: *mut _TypeFilter, 
        filterCriteria: VARIANT, 
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetEvent(
        name: BSTR, 
        bindingAttr: BindingFlags,
        pRetVal: *mut *mut _EventInfo,
    ) -> HRESULT,
    fn GetEvents(
        pRetVal: *mut *mut SAFEARRAY ,
    ) -> HRESULT,
    fn GetEvents_2(
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetNestedTypes(
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetNestedType(
        name: BSTR, 
        bindingAttr: BindingFlags,
        pRetVal: *mut *mut _Type,
    ) -> HRESULT,
    fn GetMember(
        name: BSTR,
        Type_: MemberTypes, 
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetDefaultMembers(
        pRetVal: *mut *mut SAFEARRAY ,
    ) -> HRESULT,
    fn FindMembers(
        MemberType: MemberTypes, 
        bindingAttr: BindingFlags, 
        filter: *mut _MemberFilter, 
        filterCriteria: VARIANT, 
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetElementType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn IsSubclassOf(
        c: *mut _Type ,
        pRetVal: *mut VARIANT_BOOL ,
    ) -> HRESULT,
    fn IsInstanceOfType(
        o: VARIANT, 
        pRetVal: *mut VARIANT_BOOL, 
    ) -> HRESULT,
    fn IsAssignableFrom(
        c: *mut _Type ,
        pRetVal: *mut VARIANT_BOOL ,
    ) -> HRESULT,
    fn GetInterfaceMap(
        interfaceType: *mut _Type ,
        pRetVal: *mut InterfaceMapping ,
    ) -> HRESULT,
    fn GetMethod(
        name: BSTR,
        bindingAttr: BindingFlags, 
        Binder: *mut _Binder, 
        types: *mut SAFEARRAY, 
        modifiers: *mut SAFEARRAY, 
        pRetVal: *mut *mut _MethodInfo, 
    ) -> HRESULT,
    fn GetMethod_2(
        name: BSTR,
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut _MethodInfo, 
    ) -> HRESULT,
    fn GetMethods(
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut _MethodInfo, 
    ) -> HRESULT,
    fn GetField(
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetFields(
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetProperty(
        name: BSTR,
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut _PropertyInfo,
    ) -> HRESULT,
    fn GetProperty_2(
        name: BSTR,
        bindingAttr: BindingFlags, 
        Binder: *mut _Binder,
        returnType: *mut _Type,
        types: *mut SAFEARRAY, 
        modifiers: *mut SAFEARRAY,
        pRetVal: *mut *mut _PropertyInfo,
    ) -> HRESULT,
    fn GetProperties(
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetMember_2(
        name: BSTR,
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetMembers(
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn InvokeMember(
        name: BSTR, 
        invokeAttr: BindingFlags, 
        Binder: *mut _Binder,
        Target: VARIANT, 
        args: *mut SAFEARRAY, 
        modifiers: *mut SAFEARRAY, 
        culture: *mut _CultureInfo, 
        namedParameters: *mut SAFEARRAY, 
        pRetVal: *mut VARIANT,
    )-> HRESULT,
    fn get_UnderlyingSystemType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn InvokeMember_2(
        name: BSTR, 
        invokeAttr: BindingFlags, 
        Binder: *mut _Binder,
        Target: VARIANT, 
        args: *mut SAFEARRAY, 
        culture: *mut _CultureInfo, 
        namedParameters: *mut SAFEARRAY, 
        pRetVal: *mut VARIANT,
    )-> HRESULT,
    fn InvokeMember_3(
        name: BSTR, 
        invokeAttr: BindingFlags, 
        Binder: *mut _Binder,
        Target: VARIANT, 
        args: *mut SAFEARRAY, 
        namedParameters: *mut SAFEARRAY, 
        pRetVal: *mut VARIANT,
    )-> HRESULT,
    fn GetConstructor(
        bindingAttr: BindingFlags, 
        Binder: *mut _Binder, 
        callConvention: CallingConventions,
        types: *mut SAFEARRAY, 
        modifiers: *mut SAFEARRAY, 
        pRetVal: *mut _ConstructorInfo,
    ) -> HRESULT,
    fn GetConstructor_2(
        bindingAttr: BindingFlags, 
        Binder: *mut _Binder, 
        types: *mut SAFEARRAY, 
        modifiers: *mut SAFEARRAY, 
        pRetVal: *mut _ConstructorInfo,
    ) -> HRESULT,
    fn GetConstructor_3(
        types: *mut SAFEARRAY, 
        pRetVal: *mut _ConstructorInfo,
    ) -> HRESULT,
    fn GetConstructors_2(
        pRetVal: *mut *mut SAFEARRAY ,
    ) -> HRESULT,
    fn get_TypeInitializer(
        pRetVal: *mut *mut _ConstructorInfo ,
    ) -> HRESULT,
    fn GetMethod_3(
        name: BSTR,
        bindingAttr: BindingFlags, 
        Binder: *mut _Binder, 
        callConvention: CallingConventions,
        types: *mut SAFEARRAY, 
        modifiers: *mut SAFEARRAY, 
        pRetVal: *mut *mut _MethodInfo,
    ) -> HRESULT,
    fn GetMethod_4(
        name: BSTR,
        types: *mut SAFEARRAY, 
        modifiers: *mut SAFEARRAY, 
        pRetVal: *mut *mut _MethodInfo,
    ) -> HRESULT,
    fn GetMethod_5(
        name: BSTR,
        types: *mut SAFEARRAY, 
        pRetVal: *mut *mut _MethodInfo,
    ) -> HRESULT,
    fn GetMethod_6(
        name: BSTR,
        pRetVal: *mut *mut _MethodInfo,
    ) -> HRESULT,
    fn GetMethods_2(
        pRetVal: *mut *mut SAFEARRAY ,
    ) -> HRESULT,
    fn GetField_2(
        name: BSTR, 
        pRetVal: *mut *mut _FieldInfo,
    ) -> HRESULT,
    fn GetFields_2(
        pRetVal: *mut *mut SAFEARRAY ,
    ) -> HRESULT,
    fn GetInterface_2(
        name: BSTR, 
        pRetVal: *mut *mut _Type,
    ) -> HRESULT,
    fn GetEvent_2(
        name: BSTR, 
        pRetVal: *mut *mut _EventInfo, 
    ) -> HRESULT,
    fn GetProperty_3(
        name: BSTR,
        returnType: *mut _Type,
        types: *mut SAFEARRAY, 
        modifiers: *mut SAFEARRAY,
        pRetVal: *mut *mut _PropertyInfo,
    ) -> HRESULT,
    fn GetProperty_4(
        name: BSTR,
        returnType: *mut _Type,
        types: *mut SAFEARRAY, 
        pRetVal: *mut *mut _PropertyInfo,
    ) -> HRESULT,
    fn GetProperty_5(
        name: BSTR,
        types: *mut SAFEARRAY, 
        pRetVal: *mut *mut _PropertyInfo,
    ) -> HRESULT,
    fn GetProperty_6(
        name: BSTR,
        returnType: *mut _Type,
        pRetVal: *mut *mut _PropertyInfo,
    ) -> HRESULT,
    fn GetProperty_7(
        name: BSTR,
        pRetVal: *mut *mut _PropertyInfo,
    ) -> HRESULT,
    fn GetProperties_2(
        pRetVal: *mut *mut SAFEARRAY ,
    ) -> HRESULT,
    fn GetNestedTypes_2(
        pRetVal: *mut *mut SAFEARRAY ,
    ) -> HRESULT,
    fn GetNestedType_2(
        name: BSTR, 
        pRetVal: *mut *mut _Type,
    ) -> HRESULT,
    fn GetMember_3(
        name: BSTR,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetMembers_2(
        pRetVal: *mut *mut SAFEARRAY ,
    ) -> HRESULT,
    fn get_Attributes(
        pRetVal: TypeAttributes ,
    ) -> HRESULT,
    fn get_IsNotPublic(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsPublic(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsNestedPublic(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsNestedPrivate(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsNestedFamily(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsNestedAssembly(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsNestedFamANDAssem(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsNestedFamORAssem(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsAutoLayout(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsLayoutSequential(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsExplicitLayout(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsClass(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsInterface(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsValueType(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsAbstract(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsSealed(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsEnum(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsSpecialName(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsImport(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsSerializable(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsAnsiClass(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsUnicodeClass(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsAutoClass(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsArray(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsByRef(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsPointer(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsPrimitive(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsCOMObject(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_HasElementType(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsContextful(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsMarshalByRef(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn Equals_2(
        o: *mut _Type ,
        pRetVal: *mut VARIANT_BOOL ,
    ) -> HRESULT,
}}


RIDL!{#[uuid(0xf1c3bf77, 0xc3e4, 0x11d3, 0x88, 0xe7, 0x00, 0x90, 0x27, 0x54, 0xc4, 0x3a)]
interface ITypeLibExporterNotifySink(ITypeLibExporterNotifySinkVtbl): IUnknown(IUnknownVtbl)   
{
    fn InteropServices_ReportEvent(
        eventKind: ExporterEventKind,
        eventCode: c_long,
        eventMsg: BSTR,
    ) -> HRESULT,
    fn ResolveRef(
        Assembly: *mut _Assembly,
        pRetVal: *mut *mut IUnknown,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xf1c3bf78, 0xc3e4, 0x11d3, 0x88, 0xe7, 0x00, 0x90, 0x27, 0x54, 0xc4, 0x3a)]
interface ITypeLibConverter(ITypeLibConverterVtbl): IUnknown(IUnknownVtbl)   
{
    fn ConvertTypeLibToAssembly(
        typeLib: *mut IUnknown, 
        asmFileName: BSTR, 
        flags: TypeLibImporterFlags, 
        notifySink: *mut ITypeLibImporterNotifySink, 
        publicKey: *mut SAFEARRAY,
        keyPair: *mut _StrongNameKeyPair, 
        asmNamespace: BSTR,
        asmVersion: *mut _Version, 
        pRetVal: *mut *mut _AssemblyBuilder,
    ) -> HRESULT,
    fn ConvertAssemblyToTypeLib(
        assembly: *mut _Assembly,
        typeLibName: BSTR, 
        flags: TypeLibExporterFlags, 
        notifySink: *mut ITypeLibExporterNotifySink,
        pRetVal: *mut *mut IUnknown,
    ) -> HRESULT,
    fn GetPrimaryInteropAssembly(
        G: GUID, 
        major: c_long, 
        minor: c_long, 
        lcid: LCID, 
        asmName: *mut BSTR, 
        asmCodeBase: *mut BSTR, 
        pRetVal: *mut VARIANT_BOOL, 
    ) -> HRESULT, 
    fn ConvertTypeLibToAssembly_2(
        typeLib: *mut IUnknown, 
        asmFileName: BSTR, 
        flags: c_long, 
        notifySink: *mut ITypeLibImporterNotifySink, 
        publicKey: *mut SAFEARRAY,
        keyPair: *mut _StrongNameKeyPair, 
        unsafeInterfaces: VARIANT_BOOL, 
        pRetVal: *mut *mut _AssemblyBuilder,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x6240837a, 0x707f, 0x3181, 0x8e, 0x98, 0xa3, 0x6a, 0xe0, 0x86, 0x76, 0x6b)]
interface _MethodBase(_MethodBaseVtbl): IUnknown(IUnknownVtbl)   
{
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
    fn get_ToString(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn Equals(
        other: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetHashCode(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn GetType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_MemberType(
        pRetVal: MemberTypes ,
    ) -> HRESULT,
    fn get_name(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_DeclaringType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_ReflectedType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn GetCustomAttributes(
        attributeType: *mut _Type, 
        inherit: VARIANT_BOOL, 
        pRetVal: *mut *mut SAFEARRAY,
    ) ->HRESULT,
    fn GetCustomAttributes_2(
        inherit: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn IsDefined(
        attributeType: *mut _Type, 
        inherit: VARIANT_BOOL, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetParameters(
        pRetVal: *mut *mut SAFEARRAY ,
    ) -> HRESULT,
    fn GetMethodImplementationFlags(
        pRetVal: MethodImplAttributes ,
    ) -> HRESULT,
    fn get_MethodHandle(
        pRetVal: RuntimeMethodHandle ,
    ) -> HRESULT,
    fn get_Attributes(
        pRetVal: MethodAttributes ,
    ) -> HRESULT,
    fn get_CallingConvention(
        pRetVal: CallingConventions ,
    ) -> HRESULT,
    fn Invoke_2(
        obj: VARIANT, 
        invokeAttr: BindingFlags, 
        Binder: *mut _Binder, 
        parameters: *mut SAFEARRAY,
        culture: *mut _CultureInfo,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn get_IsPublic(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsPrivate(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsFamily(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsAssembly(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsFamilyAndAssembly(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsFamilyOrAssembly(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsStatic(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsFinal(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsVirtual(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsHideBySig(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsAbstract(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsSpecialName(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsConstructor(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn Invoke_3(
        obj: VARIANT, 
        parameters: *mut SAFEARRAY,
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
}}


RIDL!{#[uuid(0xffcc1b5d, 0xecb8, 0x38dd, 0x9b, 0x01, 0x3d, 0xc8, 0xab, 0xc2, 0xaa, 0x5f)]
interface _MethodInfo(_MethodInfoVtbl): IUnknown(IUnknownVtbl)   
{
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
    fn get_ToString(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn Equals(
        other: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetHashCode(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn GetType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_MemberType(
        pRetVal: MemberTypes ,
    ) -> HRESULT,
    fn get_name(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_DeclaringType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_ReflectedType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn GetCustomAttributes(
        attributeType: *mut _Type, 
        inherit: VARIANT_BOOL, 
        pRetVal: *mut *mut SAFEARRAY,
    ) ->HRESULT,
    fn GetCustomAttributes_2(
        inherit: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn IsDefined(
        attributeType: *mut _Type, 
        inherit: VARIANT_BOOL, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetParameters(
        pRetVal: *mut *mut SAFEARRAY ,
    ) -> HRESULT,
    fn GetMethodImplementationFlags(
        pRetVal: MethodImplAttributes ,
    ) -> HRESULT,
    fn get_MethodHandle(
        pRetVal: RuntimeMethodHandle ,
    ) -> HRESULT,
    fn get_Attributes(
        pRetVal: MethodAttributes ,
    ) -> HRESULT,
    fn get_CallingConvention(
        pRetVal: CallingConventions ,
    ) -> HRESULT,
    fn Invoke_2(
        obj: VARIANT, 
        invokeAttr: BindingFlags, 
        Binder: *mut _Binder, 
        parameters: *mut SAFEARRAY, 
        culture: *mut _CultureInfo, 
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn get_IsPublic(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsPrivate(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsFamily(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsAssembly(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsFamilyAndAssembly(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsFamilyOrAssembly(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsStatic(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsFinal(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsVirtual(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsHideBySig(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsAbstract(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsSpecialName(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsConstructor(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn Invoke_3(
        obj: VARIANT, 
        parameters: *mut SAFEARRAY, 
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn get_returnType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_ReturnTypeCustomAttributes(
        pRetVal: *mut *mut ICustomAttributeProvider ,
    ) -> HRESULT,
    fn GetBaseDefinition(
        pRetVal: *mut *mut _MethodInfo ,
    ) -> HRESULT,
}}


RIDL!{#[uuid(0x9de59c64, 0xd889, 0x35a1, 0xb8, 0x97, 0x58, 0x7d, 0x74, 0x46, 0x9e, 0x5b)]
interface _EventInfo(_EventInfoVtbl): IUnknown(IUnknownVtbl)   
{
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
    fn get_ToString(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn Equals(
        other: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetHashCode(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn GetType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_MemberType(
        pRetVal: MemberTypes ,
    ) -> HRESULT,
    fn get_name(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_DeclaringType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_ReflectedType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn GetCustomAttributes(
        attributeType: *mut _Type, 
        inherit: VARIANT_BOOL, 
        pRetVal: *mut *mut SAFEARRAY,
    ) ->HRESULT,
    fn GetCustomAttributes_2(
        inherit: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn IsDefined(
        attributeType: *mut _Type, 
        inherit: VARIANT_BOOL, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetAddMethod(
        nonPublic: VARIANT_BOOL, 
        pRetVal: *mut *mut _MethodInfo, 
    ) -> HRESULT,
    fn GetRemoveMethod(
        nonPublic: VARIANT_BOOL, 
        pRetVal: *mut *mut _MethodInfo, 
    ) -> HRESULT,
    fn GetRaiseMethod(
        nonPublic: VARIANT_BOOL, 
        pRetVal: *mut *mut _MethodInfo, 
    ) -> HRESULT,
    fn get_Attributes(
        pRetVal: EventAttributes ,
    ) -> HRESULT,
    fn GetAddMethod_2(
        pRetVal: *mut *mut _MethodInfo ,
    ) -> HRESULT,
    fn GetRemoveMethod_2(
        pRetVal: *mut *mut _MethodInfo ,
    ) -> HRESULT,
    fn GetRaiseMethod_2(
        pRetVal: *mut *mut _MethodInfo ,
    ) -> HRESULT,
    fn AddEventHandler(
        Target: VARIANT, 
        handler: *mut _Delegate, 
    ) -> HRESULT,
    fn RemoveEventHandler(
        Target: VARIANT, 
        handler: *mut _Delegate, 
    ) -> HRESULT,
    fn get_EventHandlerType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_IsSpecialName(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsMulticast(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xe9a19478, 0x9646, 0x3679, 0x9b, 0x10, 0x84, 0x11, 0xae, 0x1f, 0xd5, 0x7d)]
interface _ConstructorInfo(_ConstructorInfoVtbl): IUnknown(IUnknownVtbl)   
{
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
    fn get_ToString(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn Equals(
        other: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetHashCode(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn GetType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_MemberType(
        pRetVal: MemberTypes ,
    ) -> HRESULT,
    fn get_name(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_DeclaringType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_ReflectedType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn GetCustomAttributes(
        attributeType: *mut _Type, 
        inherit: VARIANT_BOOL, 
        pRetVal: *mut *mut SAFEARRAY,
    ) ->HRESULT,
    fn GetCustomAttributes_2(
        inherit: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn IsDefined(
        attributeType: *mut _Type, 
        inherit: VARIANT_BOOL, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetParameters(
        pRetVal: *mut *mut SAFEARRAY ,
    ) -> HRESULT,
    fn GetMethodImplementationFlags(
        pRetVal: MethodImplAttributes ,
    ) -> HRESULT,
    fn get_MethodHandle(
        pRetVal:  RuntimeMethodHandle ,
    ) -> HRESULT,
    fn get_Attributes(
        pRetVal:  MethodAttributes ,
    ) -> HRESULT,
    fn get_CallingConvention(
        pRetVal:  CallingConventions ,
    ) -> HRESULT,
    fn Invoke_2(
        obj: VARIANT, 
        invokeAttr: BindingFlags, 
        Binder: *mut _Binder, 
        parameters: *mut SAFEARRAY,
        culture: *mut _CultureInfo,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn get_IsPublic(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsPrivate(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsFamily(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsAssembly(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsFamilyAndAssembly(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsFamilyOrAssembly(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsStatic(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsFinal(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsVirtual(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsHideBySig(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsAbstract(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsSpecialName(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsConstructor(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn Invoke_3(
        obj: VARIANT, 
        parameters: *mut SAFEARRAY,
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
    fn Invoke_4(
        invokeAttr: BindingFlags, 
        Binder: *mut _Binder, 
        parameters: *mut SAFEARRAY,
        culture: *mut _CultureInfo,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn Invoke_5(
        parameters: *mut SAFEARRAY ,
        pRetVal: *mut VARIANT ,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x8a7c1442, 0xa9fb, 0x366b, 0x80, 0xd8, 0x49, 0x39, 0xff, 0xa6, 0xdb, 0xe0)]
interface _FieldInfo(_FieldInfoVtbl): IUnknown(IUnknownVtbl)   
{
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
    fn get_ToString(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn Equals(
        other: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetHashCode(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn GetType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_MemberType(
        pRetVal: MemberTypes ,
    ) -> HRESULT,
    fn get_name(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_DeclaringType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_ReflectedType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn GetCustomAttributes(
        attributeType: *mut _Type, 
        inherit: VARIANT_BOOL, 
        pRetVal: *mut *mut SAFEARRAY,
    ) ->HRESULT,
    fn GetCustomAttributes_2(
        inherit: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn IsDefined(
        attributeType: *mut _Type, 
        inherit: VARIANT_BOOL, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_FieldType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn GetValue(
        obj: VARIANT, 
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
    fn GetValueDirect(
        obj: VARIANT, 
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
    fn SetValue(
        obj: VARIANT, 
        val: VARIANT, 
        invokeAttr: BindingFlags, 
        Binder: *mut _Binder, 
        culture: *mut _CultureInfo,
    ) -> HRESULT,
    fn SetValueDirect(
        obj: VARIANT, 
        val: VARIANT,
    ) -> HRESULT,
    fn get_FieldHandle(
        pRetVal: RuntimeFieldHandle ,
    ) -> HRESULT,
    fn get_Attributes(
        pRetVal: FieldAttributes ,
    ) -> HRESULT,
    fn SetValue_2(
        obj: VARIANT, 
        val: VARIANT,
    ) -> HRESULT,
    fn get_IsPublic(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsPrivate(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsFamily(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsAssembly(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsFamilyAndAssembly(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsFamilyOrAssembly(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsStatic(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsInitOnly(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsLiteral(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsNotSerialized(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsSpecialName(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsPinvokeImpl(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xf59ed4e4, 0xe68f, 0x3218, 0xbd, 0x77, 0x06, 0x1a, 0xa8, 0x28, 0x24, 0xbf)]
interface _PropertyInfo(_PropertyInfoVtbl): IUnknown(IUnknownVtbl)   
{
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
    fn get_ToString(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn Equals(
        other: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetHashCode(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn GetType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_MemberType(
        pRetVal: MemberTypes ,
    ) -> HRESULT,
    fn get_name(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_DeclaringType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_ReflectedType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn GetCustomAttributes(
        attributeType: *mut _Type, 
        inherit: VARIANT_BOOL, 
        pRetVal: *mut *mut SAFEARRAY,
    ) ->HRESULT,
    fn GetCustomAttributes_2(
        inherit: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn IsDefined(
        attributeType: *mut _Type, 
        inherit: VARIANT_BOOL, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_PropertyType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn GetValue(
        obj: VARIANT, 
        index: *mut SAFEARRAY, 
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn GetValue_2(
        obj: VARIANT,
        invokeAttr: BindingFlags, 
        Binder: *mut _Binder, 
        index: *mut SAFEARRAY, 
        culture: *mut _CultureInfo,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn SetValue(
        obj: VARIANT, 
        val: VARIANT, 
        index: *mut SAFEARRAY,
    ) -> HRESULT,
    fn SetValue_2(
        obj: VARIANT, 
        val: VARIANT, 
        invokeAttr: BindingFlags, 
        Binder: *mut _Binder, 
        index: *mut SAFEARRAY, 
        culture: *mut _CultureInfo,
    ) -> HRESULT,
    fn GetAccessors(
        nonPublic: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetGetMethod(
        nonPublic: VARIANT_BOOL, 
        pRetVal: *mut *mut _MethodInfo, 
    ) -> HRESULT,
    fn GetSetMethod(
        nonPublic: VARIANT_BOOL, 
        pRetVal: *mut *mut _MethodInfo, 
    ) -> HRESULT,
    fn GetIndexParameters(
        pRetVal: *mut *mut SAFEARRAY ,
    ) -> HRESULT,
    fn get_Attributes(
        pRetVal: PropertyAttributes ,
    ) -> HRESULT,
    fn get_CanRead(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_CanWrite(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn GetAccessors_2(
        pRetVal: *mut *mut SAFEARRAY ,
    ) -> HRESULT,
    fn GetGetMethod_2(
        pRetVal: *mut *mut _MethodInfo ,
    ) -> HRESULT,
    fn GetSetMethod_2(
        pRetVal: *mut *mut _MethodInfo ,
    ) -> HRESULT,
    fn get_IsSpecialName(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xf1c3bf76, 0xc3e4, 0x11d3, 0x88, 0xe7, 0x00, 0x90, 0x27, 0x54, 0xc4, 0x3a)]
interface ITypeLibImporterNotifySink(ITypeLibImporterNotifySinkVtbl): IUnknown(IUnknownVtbl)  
{
    fn InteropServices_ReportEvent(
        eventKind: ImporterEventKind, 
        eventCode: c_long, 
        eventMsg: BSTR, 
    ) -> HRESULT,
    fn ResolveRef(
		typeLib: *mut IUnknown ,
		pRetVal: *mut *mut  _Assembly ,
	) -> HRESULT,
}}