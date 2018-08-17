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

use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::SAFEARRAY;

RIDL!{#[uuid(0xcc18fd4d, 0xaa2d, 0x3ab4, 0x98, 0x48, 0x58, 0x4b, 0xba, 0xe4, 0xab, 0x44)]
interface IFieldInfo(IFieldInfoVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_FieldNames(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn put_FieldNames(
        pRetVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_FieldTypes(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn put_FieldTypes(
        pRetVal: *mut SAFEARRAY,
    ) -> HRESULT,
}}