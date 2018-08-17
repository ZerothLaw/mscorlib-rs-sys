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

use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::{SAFEARRAY};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

 use system::IntPtr;

RIDL!{#[uuid(0x00020404, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IEnumVARIANT(IEnumVARIANTVtbl): IUnknown(IUnknownVtbl){
    fn Next(
        celt: c_long, 
        rgVar: *mut *mut SAFEARRAY, //object[] MarshalAs(UnmanagedType.LPArray, SizeParamIndex=0)
        pceltFetched: IntPtr,
        pRetVal: *mut c_long, 
    ) -> HRESULT,
    fn Skip(
        celt: c_long, 
        pRetVal: *mut c_long, 
    ) -> HRESULT,
    fn Reset(
        pRetVal: *mut c_long, 
    ) -> HRESULT,
    fn Clone(
        pRetVal: *mut *mut IEnumVARIANT, 
    ) -> HRESULT,
}}