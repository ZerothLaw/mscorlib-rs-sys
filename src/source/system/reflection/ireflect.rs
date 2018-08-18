// ireflect.rs - MIT License
//  MIT License
//  Copyright (c) 2018 Tyler Laing (ZerothLaw)
// 
//  Permission is hereby granted, free of charge, to any person obtaining a copy
//  of this software and associated documentation files (the "Software"), to deal
//  in the Software without restriction, including without limitation the rights
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//  copies of the Software, and to permit persons to whom the Software is
//  furnished to do so, subject to the following conditions:
// 
//  The above copyright notice and this permission notice shall be included in all
//  copies or substantial portions of the Software.
// 
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//  SOFTWARE.

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