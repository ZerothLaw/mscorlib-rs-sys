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

use winapi::ctypes::c_long;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY, VARIANT};
use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};
use winapi::shared::winerror::HRESULT;

use system::globalization::_CultureInfo;
use system::reflection::{BindingFlags,_FieldInfo, _MethodBase, _PropertyInfo, _Type};

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