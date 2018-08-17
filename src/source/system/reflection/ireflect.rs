//    Copyright 2018 Tyler Laing
// 
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
// 
//        http://www.apache.org/licenses/LICENSE-2.0
// 
//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.


use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::BSTR;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY, VARIANT};

use system::globalization::_CultureInfo;
use system::reflection::{BindingFlags, _Binder, _FieldInfo, _MethodInfo, _PropertyInfo, _Type};

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