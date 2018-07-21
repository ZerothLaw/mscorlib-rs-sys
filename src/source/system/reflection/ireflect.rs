use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::SAFEARRAY;
use winapi::um::oaidl::VARIANT;
use winapi::shared::wtypes::BSTR;

use system::reflection::binder::_Binder;
use system::reflection::cominterfaces::_Type;
use system::reflection::bindingflags::BindingFlags;
use system::reflection::cominterfaces::_PropertyInfo;
use system::reflection::cominterfaces::_FieldInfo;
use system::reflection::cominterfaces::_MethodInfo;

use dispatch::_CultureInfo;

RIDL!{#[uuid(0xafbf15e5, 0xc37c, 0x11d2, 0xb8, 0x8e, 0x00, 0xa0, 0xc9, 0xb4, 0x71, 0xb8)]
interface IReflect(IReflectVtbl): IDispatch(IDispatchVtbl)  
{
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
        pRetVal: *mut *mut SAFEARRAY, //MethodInfo[]
    ) -> HRESULT,
    fn GetField(
        name: BSTR, 
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut _FieldInfo, 
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
        types: *mut SAFEARRAY, 
        modifiers: *mut SAFEARRAY, 
        pRetVal: *mut *mut _PropertyInfo,
    ) -> HRESULT,
    fn GetProperties(
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut SAFEARRAY, //PropertyInfo[]
    ) -> HRESULT,
    fn GetMember(
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
        args: *mut SAFEARRAY, 
        modifiers: *mut SAFEARRAY, 
        culture: *mut _CultureInfo, 
        namedParameters: *mut SAFEARRAY, 
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn get_UnderlyingSystemType(
		pRetVal: *mut *mut  _Type ,
	) -> HRESULT,
}}