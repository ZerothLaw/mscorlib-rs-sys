
use winapi::ctypes::{c_long,};

use winapi::shared::guiddef::{GUID, REFIID};
use winapi::shared::minwindef::{WORD,UINT};
use winapi::shared::ntdef::{LCID};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};
use winapi::shared::wtypesbase::LPOLESTR;

use winapi::um::oaidl::{DISPID, DISPPARAMS, EXCEPINFO, ITypeInfo, SAFEARRAY, VARIANT};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

use system::_Delegate;
use system::_Version;
use system::globalization::_CultureInfo;
use system::io::_Stream;
use system::io::_FileStream;
use system::RuntimeTypeHandle;
use system::reflection::_Binder;
use system::reflection::_ManifestResourceInfo;
use system::reflection::_ModuleResolveEventHandler;
use system::reflection::MemberTypes;
use system::reflection::BindingFlags;
use system::reflection::CallingConventions;
use system::reflection::EventAttributes;
use system::reflection::FieldAttributes;
use system::reflection::ICustomAttributeProvider;
use system::reflection::MethodAttributes;
use system::reflection::MethodImplAttributes;
use system::reflection::PropertyAttributes;
use system::reflection::TypeAttributes;


use system::runtime::serialization::_SerializationInfo;
use system::runtime::serialization::StreamingContext;

use system::{RuntimeFieldHandle, RuntimeMethodHandle};
use system::reflection::InterfaceMapping;
use system::reflection::_MemberFilter;
use system::reflection::_TypeFilter;

use system::security::policy::_Evidence;

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
    fn ToString_(
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
    fn get_Name(
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
        pRetVal: *mut *mut SAFEARRAY, //Object[]
    ) ->HRESULT,
    fn GetCustomAttributes_2(
        inherit: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY, //Object[]
    ) -> HRESULT,
    fn IsDefined(
        attributeType: *mut _Type, 
        inherit: VARIANT_BOOL, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Guid(
        pRetVal: *mut *mut GUID ,
    ) -> HRESULT,
    fn get_Module(
        pRetVal: *mut *mut _Module ,
    ) -> HRESULT,
    fn get_Assembly(
        pRetVal: *mut *mut _Assembly ,
    ) -> HRESULT,
    fn get_TypeHandle(
        pRetVal: *mut *mut RuntimeTypeHandle ,
    ) -> HRESULT,
    fn get_FullName(
        pRetVal: *mut *mut BSTR,
    ) -> HRESULT,
    fn get_Namespace(
        pRetVal: *mut *mut BSTR,
    ) -> HRESULT,
    fn get_AssemblyQualifiedName(
        pRetVal: *mut *mut BSTR,
    ) -> HRESULT,
    fn GetArrayRank(
        pRetVal: *mut *mut c_long,
    ) -> HRESULT,
    fn get_BaseType(
        pRetVal: *mut *mut _Type,
    ) -> HRESULT,
    fn GetConstructors(
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut SAFEARRAY, //ConstructorInfo[]
    ) -> HRESULT,
    fn GetInterface(
        name: BSTR, 
        ignoreCase: VARIANT_BOOL,
        pRetVal: *mut *mut _Type,
    ) -> HRESULT,
    fn GetInterfaces(
        pRetVal: *mut *mut SAFEARRAY, //Type[]
    ) -> HRESULT,
    fn FindInterfaces(
        filter: *mut _TypeFilter, 
        filterCriteria: VARIANT, 
        pRetVal: *mut *mut SAFEARRAY, //Type[]
    ) -> HRESULT,
    fn GetEvent(
        name: BSTR, 
        bindingAttr: BindingFlags,
        pRetVal: *mut *mut _EventInfo,
    ) -> HRESULT,
    fn GetEvents(
        pRetVal: *mut *mut SAFEARRAY, //EventInfo[]
    ) -> HRESULT,
    fn GetEvents_2(
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut SAFEARRAY, //EventInfo[]
    ) -> HRESULT,
    fn GetNestedTypes(
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut SAFEARRAY, //Type[]
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
        pRetVal: *mut *mut SAFEARRAY, //MemberInfo[]
    ) -> HRESULT,
    fn GetDefaultMembers(
        pRetVal: *mut *mut SAFEARRAY, //MemberInfo[]
    ) -> HRESULT,
    fn FindMembers(
        MemberType: MemberTypes, 
        bindingAttr: BindingFlags, 
        filter: *mut _MemberFilter, 
        filterCriteria: VARIANT, 
        pRetVal: *mut *mut SAFEARRAY, //MemberInfo[]
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
        pRetVal: *mut *mut InterfaceMapping ,
    ) -> HRESULT,
    fn GetMethod(
        name: BSTR,
        bindingAttr: BindingFlags, 
        Binder: *mut _Binder, 
        types: *mut SAFEARRAY, //Type[]
        modifiers: *mut SAFEARRAY, //ParameterModifier[]
        pRetVal: *mut *mut _MethodInfo, 
    ) -> HRESULT,
    fn GetMethod_2(
        name: BSTR,
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut _MethodInfo, 
    ) -> HRESULT,
    fn GetMethods(
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut SAFEARRAY,  //MethodInfo[]
    ) -> HRESULT,
    fn GetField(
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetFields(
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut SAFEARRAY, //FieldInfo[]
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
        types: *mut SAFEARRAY, //Type[]
        modifiers: *mut SAFEARRAY, //ParameterModifier[]
        pRetVal: *mut *mut _PropertyInfo,
    ) -> HRESULT,
    fn GetProperties(
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut SAFEARRAY, //PropertyInfo[]
    ) -> HRESULT,
    fn GetMember_2(
        name: BSTR,
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut SAFEARRAY, //MemberInfo[]
    ) -> HRESULT,
    fn GetMembers(
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut SAFEARRAY, //MemberInfo[]
    ) -> HRESULT,
    fn InvokeMember(
        name: BSTR, 
        invokeAttr: BindingFlags, 
        Binder: *mut _Binder,
        Target: VARIANT, 
        args: *mut SAFEARRAY, //Object[]
        modifiers: *mut SAFEARRAY, //ParameterModifier[]
        culture: *mut _CultureInfo, 
        namedParameters: *mut SAFEARRAY, //String[]
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
        args: *mut SAFEARRAY, //Object[]
        culture: *mut _CultureInfo, 
        namedParameters: *mut SAFEARRAY, 
        pRetVal: *mut *mut VARIANT,
    )-> HRESULT,
    fn InvokeMember_3(
        name: BSTR, 
        invokeAttr: BindingFlags, 
        Binder: *mut _Binder,
        Target: VARIANT, 
        args: *mut SAFEARRAY, //Object[]
        namedParameters: *mut SAFEARRAY, 
        pRetVal: *mut *mut VARIANT,
    )-> HRESULT,
    fn GetConstructor(
        bindingAttr: BindingFlags, 
        Binder: *mut _Binder, 
        callConvention: CallingConventions,
        types: *mut SAFEARRAY, //Type[]
        modifiers: *mut SAFEARRAY, //ParameterModifier[]
        pRetVal: *mut *mut _ConstructorInfo,
    ) -> HRESULT,
    fn GetConstructor_2(
        bindingAttr: BindingFlags, 
        Binder: *mut _Binder, 
        types: *mut SAFEARRAY, 
        modifiers: *mut SAFEARRAY, 
        pRetVal: *mut *mut _ConstructorInfo,
    ) -> HRESULT,
    fn GetConstructor_3(
        types: *mut SAFEARRAY, 
        pRetVal: *mut *mut _ConstructorInfo,
    ) -> HRESULT,
    fn GetConstructors_2(
        pRetVal: *mut *mut SAFEARRAY ,
    ) -> HRESULT,
    fn get_TypeInitializer(
        pRetVal: *mut *mut _ConstructorInfo,
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
        pRetVal: *mut *mut SAFEARRAY , //MethodInfo[]
    ) -> HRESULT,
    fn GetField_2(
        name: BSTR, 
        pRetVal: *mut *mut _FieldInfo,
    ) -> HRESULT,
    fn GetFields_2(
        pRetVal: *mut *mut SAFEARRAY , //FieldInfo[]
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
        pRetVal: *mut *mut TypeAttributes ,
    ) -> HRESULT,
    fn get_IsNotPublic(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsPublic(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsNestedPublic(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsNestedPrivate(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsNestedFamily(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsNestedAssembly(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsNestedFamANDAssem(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsNestedFamORAssem(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsAutoLayout(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsLayoutSequential(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsExplicitLayout(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsClass(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsInterface(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsValueType(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsAbstract(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsSealed(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsEnum(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsSpecialName(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsImport(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsSerializable(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsAnsiClass(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsUnicodeClass(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsAutoClass(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsArray(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsByRef(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsPointer(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsPrimitive(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsCOMObject(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_HasElementType(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsContextful(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsMarshalByRef(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Equals_2(
        o: *mut _Type ,
        pRetVal: *mut VARIANT_BOOL ,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x17156360, 0x2f1a, 0x384a, 0xbc, 0x52, 0xfd, 0xe9, 0x3c, 0x21, 0x5c, 0x5b)]
interface _Assembly(_AssemblyVtbl): IDispatch(IDispatchVtbl)  
{
    fn ToString_(
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
		pRetVal: *mut *mut  _Type ,
	) -> HRESULT,
    fn get_CodeBase( //NotImplemented
        pRetVal: *mut BSTR ,
    ) -> HRESULT,
    fn get_EscapedCodeBase(
        pRetVal: *mut BSTR ,
    ) -> HRESULT,
    fn GetName( //NotImplemented
		pRetVal: *mut *mut  _AssemblyName ,
	) -> HRESULT,
    fn GetName_2( //NotImplemented
        copiedName: VARIANT_BOOL, 
        pRetVal: *mut *mut _AssemblyName,
    ) -> HRESULT,
    fn get_FullName( //NotImplemented
        pRetVal: *mut BSTR ,
    ) -> HRESULT,
    fn get_EntryPoint( //NotImplemented
		pRetVal: *mut *mut  _MethodInfo ,
	) -> HRESULT,
    fn GetType_2(
        name: BSTR, 
        pRetVal: *mut *mut _Type, 
    ) -> HRESULT,
    fn GetType_3( //NotImplemented
        name: BSTR, 
        throwOnError: VARIANT_BOOL, 
        pRetVal: *mut *mut _Type,
    ) -> HRESULT,
    fn GetExportedTypes( //NotImplemented
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn GetTypes(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn GetManifestResourceStream( //NotImplemented
        Type_: *mut _Type, 
        name: BSTR, 
        pRetVal: *mut *mut _Stream,
    ) -> HRESULT,
    fn GetManifestResourceStream_2( //NotImplemented
        name: BSTR, 
        pRetVal: *mut *mut _Stream,
    ) -> HRESULT, 
    fn GetFile( //NotImplemented
        name: BSTR, 
        pRetVal: *mut *mut _FileStream,
    ) -> HRESULT,
    fn GetFiles(  //NotImplemented
		pRetVal: *mut *mut SAFEARRAY , //FileStream[] 
	) -> HRESULT,
    fn GetFiles_2(
        getResourceModules: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY, //FileStream[]
    ) -> HRESULT,
    fn GetManifestResourceNames( //NotImplemented
		pRetVal: *mut *mut SAFEARRAY , //String[]
	) -> HRESULT,
    fn GetManifestResourceInfo( //NotImplemented
        resourceName: BSTR, 
        pRetVal: *mut *mut _ManifestResourceInfo,
    ) -> HRESULT,
    fn get_Location( //NotImplemented
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_Evidence( //NotImplemented
		pRetVal: *mut *mut  _Evidence ,
	) -> HRESULT,
    fn GetCustomAttributes(
        attributeType: *mut _Type, 
        inherit: VARIANT_BOOL, 
        pRetVal: *mut *mut SAFEARRAY, 
    ) -> HRESULT,
    fn GetCustomAttributes_2( //NotImplemented
        inherit: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn IsDefined( //NotImplemented
        attributeType: *mut _Type, 
        inherit: VARIANT_BOOL,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetObjectData( //NotImplemented
        info: *mut _SerializationInfo,
        Context: StreamingContext,
    ) -> HRESULT,
    fn add_ModuleResolve( //NotImplemented
        val: *mut _ModuleResolveEventHandler, 
    ) -> HRESULT,
    fn remove_ModuleResolve( //NotImplemented
        val: *mut _ModuleResolveEventHandler,
    ) -> HRESULT,
    fn GetType_4(
        name: BSTR, 
        throwOnError: VARIANT_BOOL, 
        ignoreCase: VARIANT_BOOL, 
        pRetVal: *mut *mut _Type,
    ) -> HRESULT,
    fn GetSatelliteAssembly( //NotImplemented
		culture: *mut  _CultureInfo ,
		pRetVal: *mut *mut  _Assembly ,
	) -> HRESULT,
    fn GetSatelliteAssembly_2( //NotImplemented
        culture: *mut _CultureInfo,
        Version: *mut _Version, 
        pRetVal: *mut *mut _Assembly,
    ) -> HRESULT,
    fn LoadModule(
        moduleName: BSTR, 
        rawModule: *mut SAFEARRAY,  //byte[]
        pRetVal: *mut *mut _Module,
    ) -> HRESULT,
    fn LoadModule_2( //NotImplemented
        moduleName: BSTR, 
        rawModule: *mut SAFEARRAY, //byte[]
        rawSymbolStore: *mut SAFEARRAY, //byte[]
        pRetVal: *mut *mut _Module,
    ) -> HRESULT,
    fn CreateInstance(
        typeName: BSTR,
        pRetVal: *mut *mut VARIANT,
    ) -> HRESULT,
    fn CreateInstance_2(
        typeName: BSTR, 
        ignoreCase: VARIANT_BOOL, 
        pRetVal: *mut *mut VARIANT,
    ) -> HRESULT,
    fn CreateInstance_3(
        typeName: BSTR, 
        ignoreCase: VARIANT_BOOL, 
        bindingAttr: BindingFlags, 
        Binder: *mut _Binder,
        args: *mut SAFEARRAY, 
        culture: *mut _CultureInfo, 
        activationAttributes: *mut SAFEARRAY, 
        pRetVal: *mut *mut VARIANT,
    ) -> HRESULT,
    fn GetLoadedModules(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn GetLoadedModules_2( //NotImplemented
        getResourceModules: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetModules(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn GetModules_2( //NotImplemented
        getResourceModules: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetModule( //NotImplemented
        name: BSTR, 
        pRetVal: *mut *mut _Module,
    ) -> HRESULT,
    fn GetReferencedAssemblies( //NotImplemented
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn get_GlobalAssemblyCache( //NotImplemented
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
}} //implemented by Assembly, which also implements IEvidenceFactory, ICustomAttributeProvider, ISerializable


RIDL!{#[uuid(0xf7102fa9, 0xcabb, 0x3a74, 0xa6, 0xda, 0xb4, 0x56, 0x7e, 0xf1, 0xb0, 0x79)]
interface _MemberInfo(_MemberInfoVtbl): IUnknown(IUnknownVtbl)   
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
    fn ToString_(
        pRetVal: *mut BSTR ,
    ) -> HRESULT,
    fn Equals(
        other: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetHashCode(
        pRetVal: *mut c_long ,
    ) -> HRESULT,
    fn GetType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_MemberType(
        pRetVal: *mut MemberTypes ,
    ) -> HRESULT,
    fn get_Name(
        pRetVal: *mut  BSTR,
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
        pRetVal: *mut BSTR ,
    ) -> HRESULT,
    fn Equals(
        other: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetHashCode(
        pRetVal: *mut c_long ,
    ) -> HRESULT,
    fn GetType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_MemberType(
        pRetVal: *mut MemberTypes ,
    ) -> HRESULT,
    fn get_Name(
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
    fn GetParameters(
        pRetVal: *mut *mut SAFEARRAY ,
    ) -> HRESULT,
    fn GetMethodImplementationFlags(
        pRetVal: *mut MethodImplAttributes ,
    ) -> HRESULT,
    fn get_MethodHandle(
        pRetVal: *mut RuntimeMethodHandle ,
    ) -> HRESULT,
    fn get_Attributes(
        pRetVal: *mut MethodAttributes ,
    ) -> HRESULT,
    fn get_CallingConvention(
        pRetVal: *mut CallingConventions ,
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
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsPrivate(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFamily(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsAssembly(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFamilyAndAssembly(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFamilyOrAssembly(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsStatic(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFinal(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsVirtual(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsHideBySig(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsAbstract(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsSpecialName(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsConstructor(
        pRetVal: *mut *mut VARIANT_BOOL,
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
    fn GetTypeInfoCount( //Not Implemented
        pcTInfo: *mut UINT,
    ) -> HRESULT,
    fn GetTypeInfo(  //Not Implemented
        iTInfo: UINT,
        lcid: LCID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetIDsOfNames(  //Not Implemented
        riid: REFIID,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        lcid: LCID,
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke(  //Not Implemented
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
        pRetVal: *mut c_long ,
    ) -> HRESULT,
    fn GetType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_MemberType(
        pRetVal: *mut MemberTypes ,
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
    fn GetParameters(
        pRetVal: *mut *mut SAFEARRAY ,
    ) -> HRESULT,
    fn GetMethodImplementationFlags(
        pRetVal: *mut MethodImplAttributes ,
    ) -> HRESULT,
    fn get_MethodHandle(
        pRetVal: *mut RuntimeMethodHandle ,
    ) -> HRESULT,
    fn get_Attributes(
        pRetVal: *mut MethodAttributes ,
    ) -> HRESULT,
    fn get_CallingConvention(
        pRetVal: *mut CallingConventions ,
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
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsPrivate(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFamily(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsAssembly(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFamilyAndAssembly(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFamilyOrAssembly(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsStatic(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFinal(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsVirtual(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsHideBySig(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsAbstract(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsSpecialName(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsConstructor(
        pRetVal: *mut *mut VARIANT_BOOL,
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

//implemented by ConstructorInfo, which also implements MethodBase
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
        pRetVal: *mut BSTR ,
    ) -> HRESULT,
    fn Equals(
        other: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetHashCode(
        pRetVal: *mut c_long ,
    ) -> HRESULT,
    fn GetType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_MemberType(
        pRetVal: *mut MemberTypes ,
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
    fn GetParameters(
        pRetVal: *mut *mut SAFEARRAY ,
    ) -> HRESULT,
    fn GetMethodImplementationFlags(
        pRetVal: *mut MethodImplAttributes ,
    ) -> HRESULT,
    fn get_MethodHandle(
        pRetVal:  *mut RuntimeMethodHandle ,
    ) -> HRESULT,
    fn get_Attributes(
        pRetVal:  *mut MethodAttributes ,
    ) -> HRESULT,
    fn get_CallingConvention(
        pRetVal:  *mut CallingConventions ,
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
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsPrivate(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFamily(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsAssembly(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFamilyAndAssembly(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFamilyOrAssembly(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsStatic(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFinal(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsVirtual(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsHideBySig(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsAbstract(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsSpecialName(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsConstructor(
        pRetVal: *mut *mut VARIANT_BOOL,
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
    fn get_ToString(
        pRetVal: *mut BSTR ,
    ) -> HRESULT,
    fn Equals(
        other: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetHashCode(
        pRetVal: *mut c_long ,
    ) -> HRESULT,
    fn GetType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_MemberType(
        pRetVal: *mut MemberTypes ,
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
    fn get_FieldType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn GetValue(
        obj: VARIANT, 
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
    fn GetValueDirect( //Not Implemented
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
    fn SetValueDirect( //Not Implemented
        obj: VARIANT, 
        val: VARIANT,
    ) -> HRESULT,
    fn get_FieldHandle(
        pRetVal:*mut  RuntimeFieldHandle ,
    ) -> HRESULT,
    fn get_Attributes(
        pRetVal: *mut FieldAttributes ,
    ) -> HRESULT,
    fn SetValue_2(
        obj: VARIANT, 
        val: VARIANT,
    ) -> HRESULT,
    fn get_IsPublic(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsPrivate(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFamily(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsAssembly(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFamilyAndAssembly(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFamilyOrAssembly(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsStatic(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsInitOnly(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsLiteral(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsNotSerialized(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsSpecialName(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsPinvokeImpl(
        pRetVal: *mut *mut VARIANT_BOOL,
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
        pRetVal: *mut BSTR ,
    ) -> HRESULT,
    fn Equals(
        other: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetHashCode(
        pRetVal: *mut c_long ,
    ) -> HRESULT,
    fn GetType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_MemberType(
        pRetVal: *mut MemberTypes ,
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
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanWrite(
        pRetVal: *mut *mut VARIANT_BOOL,
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
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
}}


RIDL!{#[uuid(0x9de59c64, 0xd889, 0x35a1, 0xb8, 0x97, 0x58, 0x7d, 0x74, 0x46, 0x9e, 0x5b)]
interface _EventInfo(_EventInfoVtbl): IUnknown(IUnknownVtbl)   
{
    fn GetTypeInfoCount( //notimplemented
        pcTInfo: *mut UINT,
    ) -> HRESULT,
    fn GetTypeInfo( //notimplemented
        iTInfo: UINT,
        lcid: LCID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetIDsOfNames( //notimplemented
        riid: REFIID,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        lcid: LCID,
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke( //notimplemented
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
        pRetVal: *mut c_long ,
    ) -> HRESULT,
    fn GetType(
        pRetVal: *mut *mut _Type ,
    ) -> HRESULT,
    fn get_MemberType(
        pRetVal: *mut MemberTypes ,
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
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsMulticast(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x993634c4, 0xe47a, 0x32cc, 0xbe, 0x08, 0x85, 0xf5, 0x67, 0xdc, 0x27, 0xd6)]
interface _ParameterInfo(_ParameterInfoVtbl) : IUnknown(IUnknownVtbl) {
    fn GetTypeInfoCount( //NotImplemented
        pcTInfo: *mut UINT,
    ) -> HRESULT,
    fn GetTypeInfo( //NotImplemented
        iTInfo: UINT,
        lcid: LCID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetIDsOfNames( //NotImplemented
        riid: REFIID,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        lcid: LCID,
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke( //NotImplemented
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


RIDL!{#[uuid(0xb42b6aac, 0x317e, 0x34d5, 0x9f, 0xa9, 0x09, 0x3b, 0xb4, 0x16, 0x0c, 0x50)]
interface _AssemblyName(_AssemblyNameVtbl) : IUnknown(IUnknownVtbl) {
    fn GetTypeInfoCount( //NotImplemented
        pcTInfo: *mut UINT,
    ) -> HRESULT,
    fn GetTypeInfo( //NotImplemented
        iTInfo: UINT,
        lcid: LCID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetIDsOfNames( //NotImplemented
        riid: REFIID,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        lcid: LCID,
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke( //NotImplemented
        dispIdMember: DISPID,
        riid: REFIID,
        lcid: LCID,
        wFlags: WORD,
        pDispParams: *mut DISPPARAMS,
        pVarResult: *mut VARIANT,
        pExcepInfo: *mut EXCEPINFO,
        puArgErr: *mut UINT,
    ) -> HRESULT,
}} //implemented by AssemblyName, which also implements ICloneable, ISerializable, IDeserializationCallback