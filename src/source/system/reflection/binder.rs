// binder.rs - MIT License
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

use winapi::ctypes::c_long;

use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY, VARIANT};

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