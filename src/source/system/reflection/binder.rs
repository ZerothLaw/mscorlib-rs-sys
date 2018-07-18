use winapi::ctypes::c_long;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::um::oaidl::{SAFEARRAY, VARIANT};
use winapi::shared::wtypes::{BSTR};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::VARIANT_BOOL;

use source::system::reflection::cominterfaces::_Type;
use source::system::reflection::cominterfaces::_PropertyInfo;
use source::system::reflection::bindingflags::BindingFlags;
use source::system::reflection::cominterfaces::_MethodBase;
use source::system::reflection::cominterfaces::_FieldInfo;

use dispatch::_CultureInfo;
RIDL!{#[uuid(0x3169ab11, 0x7109, 0x3808, 0x9a, 0x61, 0xef, 0x4b, 0xa0, 0x53, 0x4f, 0xd9)]
interface _Binder(_BinderVtbl): IDispatch(IDispatchVtbl)  
{
    fn ToString_(
        pRetVal: *mut BSTR ,
    ) -> HRESULT,
    fn Equals(
        obj: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
      fn GetHashCode(
        pRetVal: *mut c_long ,
    ) -> HRESULT,
    fn GetType(
		pRetVal: *mut *mut _Type ,
	) -> HRESULT,
    fn BindToMethod(
        bindingAttr: BindingFlags,
        match_: *mut SAFEARRAY, 
        args: *mut SAFEARRAY, 
        modifiers: *mut SAFEARRAY, 
        culture: *mut _CultureInfo, 
        names: *mut SAFEARRAY, 
        state: *mut VARIANT, 
        pRetVal: *mut *mut _MethodBase,
    ) -> HRESULT,
    fn BindToField(
        bindingAttr: BindingFlags,
        match_: *mut SAFEARRAY, 
        val: VARIANT, 
        culture: *mut _CultureInfo, 
        pRetVal: *mut *mut _FieldInfo,
    ) -> HRESULT,
    fn SelectMethod(
        bindingAttr: BindingFlags, 
        match_: *mut SAFEARRAY, 
        types: *mut SAFEARRAY, 
        modifiers: *mut SAFEARRAY, 
        pRetVal: *mut *mut _MethodBase, 
    ) -> HRESULT,
    fn SelectProperty(
        bindingAttr: BindingFlags, 
        match_: *mut SAFEARRAY,
        returnType: *mut _Type, 
        indexes: *mut SAFEARRAY, 
        modifiers: *mut SAFEARRAY, 
        pRetVal: *mut *mut _PropertyInfo, 
    ) -> HRESULT,
    fn ChangeType(
        val: VARIANT, 
        Type_: *mut _Type, 
        culture: *mut _CultureInfo, 
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
    fn ReorderArgumentArray(
        args: *mut *mut SAFEARRAY, 
        state: VARIANT,
    ) -> HRESULT, 
}}