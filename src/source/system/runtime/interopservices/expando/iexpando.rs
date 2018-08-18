// iexpando.rs - MIT License
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
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

use system::_Delegate;
use system::reflection::{_FieldInfo, _MemberInfo, _MethodInfo, _PropertyInfo};

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