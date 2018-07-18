use winapi::shared::winerror::HRESULT;

use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::shared::minwindef::UINT;
use winapi::um::oaidl::EXCEPINFO;
use winapi::um::oaidl::VARIANT;
use winapi::um::oaidl::DISPPARAMS;
use winapi::shared::minwindef::WORD;
use winapi::um::winnt::LCID;
use winapi::shared::guiddef::REFIID;
use winapi::um::oaidl::DISPID;
use winapi::shared::wtypesbase::LPOLESTR;
use winapi::um::oaidl::ITypeInfo;

RIDL!{#[uuid(0xbebb2505, 0x8b54, 0x3443, 0xae, 0xad, 0x14, 0x2a, 0x16, 0xdd, 0x9c, 0xc7)]
interface _AssemblyBuilder(_AssemblyBuilderVtbl): IUnknown(IUnknownVtbl)   
{
    fn GetTypeInfoCount( //Not Implemented
        pcTInfo: *mut UINT,
    ) -> HRESULT,
    fn GetTypeInfo( //Not Implemented
        iTInfo: UINT,
        lcid: LCID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetIDsOfNames( //Not Implemented
        riid: REFIID,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        lcid: LCID,
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke( //Not Implemented
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
//implemented by Assembly, which also implements Assembly

RIDL!{#[uuid(0xed3e4384, 0xd7e2, 0x3fa7, 0x8f, 0xfd, 0x89, 0x40, 0xd3, 0x30, 0x51, 0x9a)]
interface _ConstructorBuilder(_ConstructorBuilderVtbl): IUnknown(IUnknownVtbl)   
{
    fn GetTypeInfoCount( //Not Implemented
        pcTInfo: *mut UINT,
    ) -> HRESULT,
    fn GetTypeInfo( //Not Implemented
        iTInfo: UINT,
        lcid: LCID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetIDsOfNames( //Not Implemented
        riid: REFIID,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        lcid: LCID,
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke( //Not Implemented
        dispIdMember: DISPID,
        riid: REFIID,
        lcid: LCID,
        wFlags: WORD,
        pDispParams: *mut DISPPARAMS,
        pVarResult: *mut VARIANT,
        pExcepInfo: *mut EXCEPINFO,
        puArgErr: *mut UINT,
    ) -> HRESULT,
}} //implemented by ConstructorBuilder, which also implements ConstructorInfo

RIDL!{#[uuid(0xbe9acce8, 0xaaff, 0x3b91, 0x81, 0xae, 0x82, 0x11, 0x66, 0x3f, 0x5c, 0xad)]
interface _CustomAttributeBuilder(_CustomAttributeBuilderVtbl): IUnknown(IUnknownVtbl)   
{
    fn GetTypeInfoCount( //Not Implemented
        pcTInfo: *mut UINT,
    ) -> HRESULT,
    fn GetTypeInfo( //Not Implemented
        iTInfo: UINT,
        lcid: LCID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetIDsOfNames( //Not Implemented
        riid: REFIID,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        lcid: LCID,
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke( //Not Implemented
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
    fn GetTypeInfoCount( //Not Implemented
        pcTInfo: *mut UINT,
    ) -> HRESULT,
    fn GetTypeInfo( //Not Implemented
        iTInfo: UINT,
        lcid: LCID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetIDsOfNames( //Not Implemented
        riid: REFIID,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        lcid: LCID,
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke( //Not Implemented
        dispIdMember: DISPID,
        riid: REFIID,
        lcid: LCID,
        wFlags: WORD,
        pDispParams: *mut DISPPARAMS,
        pVarResult: *mut VARIANT,
        pExcepInfo: *mut EXCEPINFO,
        puArgErr: *mut UINT,
    ) -> HRESULT,
}} //implemented by EnumBuilder, which also inherits from TypeInfo

RIDL!{#[uuid(0xaadaba99, 0x895d, 0x3d65, 0x97, 0x60, 0xb1, 0xf1, 0x26, 0x21, 0xfa, 0xe8)]
interface _EventBuilder(_EventBuilderVtbl): IUnknown(IUnknownVtbl)   
{
    fn GetTypeInfoCount( //Not Implemented
        pcTInfo: *mut UINT,
    ) -> HRESULT,
    fn GetTypeInfo( //Not Implemented
        iTInfo: UINT,
        lcid: LCID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetIDsOfNames( //Not Implemented
        riid: REFIID,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        lcid: LCID,
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke( //Not Implemented
        dispIdMember: DISPID,
        riid: REFIID,
        lcid: LCID,
        wFlags: WORD,
        pDispParams: *mut DISPPARAMS,
        pVarResult: *mut VARIANT,
        pExcepInfo: *mut EXCEPINFO,
        puArgErr: *mut UINT,
    ) -> HRESULT,
}} //implemented by EventBuilder

RIDL!{#[uuid(0xce1a3bf5, 0x975e, 0x30cc, 0x97, 0xc9, 0x1e, 0xf7, 0x0f, 0x8f, 0x39, 0x93)]
interface _FieldBuilder(_FieldBuilderVtbl): IUnknown(IUnknownVtbl)   
{
    fn GetTypeInfoCount( //Not Implemented
        pcTInfo: *mut UINT,
    ) -> HRESULT,
    fn GetTypeInfo( //Not Implemented
        iTInfo: UINT,
        lcid: LCID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetIDsOfNames( //Not Implemented
        riid: REFIID,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        lcid: LCID,
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke( //Not Implemented
        dispIdMember: DISPID,
        riid: REFIID,
        lcid: LCID,
        wFlags: WORD,
        pDispParams: *mut DISPPARAMS,
        pVarResult: *mut VARIANT,
        pExcepInfo: *mut EXCEPINFO,
        puArgErr: *mut UINT,
    ) -> HRESULT,
}} //implemented by FieldBuilder, which also implements FieldInfo

RIDL!{#[uuid(0xa4924b27, 0x6e3b, 0x37f7, 0x9b, 0x83, 0xa4, 0x50, 0x19, 0x55, 0xe6, 0xa7)]
interface _ILGenerator(_ILGeneratorVtbl): IUnknown(IUnknownVtbl)   
{
    fn GetTypeInfoCount( //Not Implemented
        pcTInfo: *mut UINT,
    ) -> HRESULT,
    fn GetTypeInfo( //Not Implemented
        iTInfo: UINT,
        lcid: LCID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetIDsOfNames( //Not Implemented
        riid: REFIID,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        lcid: LCID,
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke( //Not Implemented
        dispIdMember: DISPID,
        riid: REFIID,
        lcid: LCID,
        wFlags: WORD,
        pDispParams: *mut DISPPARAMS,
        pVarResult: *mut VARIANT,
        pExcepInfo: *mut EXCEPINFO,
        puArgErr: *mut UINT,
    ) -> HRESULT,
}} //implemented by ILGenerator
 
RIDL!{#[uuid(0x4e6350d1, 0xa08b, 0x3dec, 0x9a, 0x3e, 0xc4, 0x65, 0xf9, 0xae, 0xec, 0x0c)]
interface _LocalBuilder(_LocalBuilderVtbl): IUnknown(IUnknownVtbl)   
{
    fn GetTypeInfoCount( //Not Implemented
        pcTInfo: *mut UINT,
    ) -> HRESULT,
    fn GetTypeInfo( //Not Implemented
        iTInfo: UINT,
        lcid: LCID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetIDsOfNames( //Not Implemented
        riid: REFIID,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        lcid: LCID,
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke( //Not Implemented
        dispIdMember: DISPID,
        riid: REFIID,
        lcid: LCID,
        wFlags: WORD,
        pDispParams: *mut DISPPARAMS,
        pVarResult: *mut VARIANT,
        pExcepInfo: *mut EXCEPINFO,
        puArgErr: *mut UINT,
    ) -> HRESULT,
}} //implemented by LocalBuilder, which implements LocalVariableInfo

RIDL!{#[uuid(0x007d8a14, 0xfdf3, 0x363e, 0x9a, 0x0b, 0xfe, 0xc0, 0x61, 0x82, 0x60, 0xa2)]
interface _MethodBuilder(_MethodBuilderVtbl): IUnknown(IUnknownVtbl)   
{
    fn GetTypeInfoCount( //Not Implemented
        pcTInfo: *mut UINT,
    ) -> HRESULT,
    fn GetTypeInfo( //Not Implemented
        iTInfo: UINT,
        lcid: LCID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetIDsOfNames( //Not Implemented
        riid: REFIID,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        lcid: LCID,
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke( //Not Implemented
        dispIdMember: DISPID,
        riid: REFIID,
        lcid: LCID,
        wFlags: WORD,
        pDispParams: *mut DISPPARAMS,
        pVarResult: *mut VARIANT,
        pExcepInfo: *mut EXCEPINFO,
        puArgErr: *mut UINT,
    ) -> HRESULT,
}} //implemented by MethodBuilder, which also implements MethodInfo

RIDL!{#[uuid(0xc2323c25, 0xf57f, 0x3880, 0x8a, 0x4d, 0x12, 0xeb, 0xea, 0x7a, 0x58, 0x52)]
interface _MethodRental(_MethodRentalVtbl): IUnknown(IUnknownVtbl)   
{
    fn GetTypeInfoCount( //Not Implemented
        pcTInfo: *mut UINT,
    ) -> HRESULT,
    fn GetTypeInfo( //Not Implemented
        iTInfo: UINT,
        lcid: LCID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetIDsOfNames( //Not Implemented
        riid: REFIID,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        lcid: LCID,
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke( //Not Implemented
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
    fn GetTypeInfoCount( //Not Implemented
        pcTInfo: *mut UINT,
    ) -> HRESULT,
    fn GetTypeInfo( //Not Implemented
        iTInfo: UINT,
        lcid: LCID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetIDsOfNames( //Not Implemented
        riid: REFIID,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        lcid: LCID,
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke( //Not Implemented
        dispIdMember: DISPID,
        riid: REFIID,
        lcid: LCID,
        wFlags: WORD,
        pDispParams: *mut DISPPARAMS,
        pVarResult: *mut VARIANT,
        pExcepInfo: *mut EXCEPINFO,
        puArgErr: *mut UINT,
    ) -> HRESULT,
}} //implemented by ModuleBuilder, which also implements Module

RIDL!{#[uuid(0x36329eba, 0xf97a, 0x3565, 0xbc, 0x07, 0x0e, 0xd5, 0xc6, 0xef, 0x19, 0xfc)]
interface _ParameterBuilder(_ParameterBuilderVtbl): IUnknown(IUnknownVtbl)   
{
    fn GetTypeInfoCount( //Not Implemented
        pcTInfo: *mut UINT,
    ) -> HRESULT,
    fn GetTypeInfo( //Not Implemented
        iTInfo: UINT,
        lcid: LCID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetIDsOfNames( //Not Implemented
        riid: REFIID,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        lcid: LCID,
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke( //Not Implemented
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
    fn GetTypeInfoCount( //Not Implemented
        pcTInfo: *mut UINT,
    ) -> HRESULT,
    fn GetTypeInfo( //Not Implemented
        iTInfo: UINT,
        lcid: LCID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetIDsOfNames( //Not Implemented
        riid: REFIID,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        lcid: LCID,
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke( //Not Implemented
        dispIdMember: DISPID,
        riid: REFIID,
        lcid: LCID,
        wFlags: WORD,
        pDispParams: *mut DISPPARAMS,
        pVarResult: *mut VARIANT,
        pExcepInfo: *mut EXCEPINFO,
        puArgErr: *mut UINT,
    ) -> HRESULT,
}} //implemented by PropertyBuilder, which also implements PropertyInfo

RIDL!{#[uuid(0x7d13dd37, 0x5a04, 0x393c, 0xbb, 0xca, 0xa5, 0xfe, 0xa8, 0x02, 0x89, 0x3d)]
interface _SignatureHelper(_SignatureHelperVtbl): IUnknown(IUnknownVtbl)   
{
    fn GetTypeInfoCount( //Not Implemented
        pcTInfo: *mut UINT,
    ) -> HRESULT,
    fn GetTypeInfo( //Not Implemented
        iTInfo: UINT,
        lcid: LCID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetIDsOfNames( //Not Implemented
        riid: REFIID,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        lcid: LCID,
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke( //Not Implemented
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
    fn GetTypeInfoCount( //Not Implemented
        pcTInfo: *mut UINT,
    ) -> HRESULT,
    fn GetTypeInfo( //Not Implemented
        iTInfo: UINT,
        lcid: LCID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetIDsOfNames( //Not Implemented
        riid: REFIID,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        lcid: LCID,
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke( //Not Implemented
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
