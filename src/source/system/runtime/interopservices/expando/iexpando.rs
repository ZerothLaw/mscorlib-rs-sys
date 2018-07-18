use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::BSTR;

use source::system::reflection::cominterfaces::_MemberInfo;
use source::system::reflection::cominterfaces::_MethodInfo;
use source::system::delegate::_Delegate;
use source::system::reflection::cominterfaces::_PropertyInfo;
use source::system::reflection::cominterfaces::_FieldInfo;

RIDL!{#[uuid(0xafbf15e6, 0xc37c, 0x11d2, 0xb8, 0x8e, 0x00, 0xa0, 0xc9, 0xb4, 0x71, 0xb8)]
interface IExpando(IExpandoVtbl): IDispatch(IDispatchVtbl)  
{
    fn AddField(
        name: BSTR, 
        pRetVal: *mut *mut _FieldInfo,
    ) -> HRESULT,
    fn AddProperty(
        name: BSTR, 
        pRetVal: *mut *mut _PropertyInfo, 
    ) -> HRESULT,
    fn AddMethod(
        name: BSTR, 
        Method: *mut _Delegate, 
        pRetVal: *mut *mut _MethodInfo, 
    ) -> HRESULT,
    fn RemoveMember(
        m: *mut _MemberInfo,
    ) -> HRESULT,
}}