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
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

use system::delegate::_Delegate;
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