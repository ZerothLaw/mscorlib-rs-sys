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
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::BSTR;

use system::security::principal::iidentity::IIdentity;

RIDL!{#[uuid(0x4283ca6c, 0xd291, 0x3481, 0x83, 0xc9, 0x95, 0x54, 0x48, 0x1f, 0xe8, 0x88)]
interface IPrincipal(IPrincipalVtbl) : IDispatch(IDispatchVtbl) {
    fn get_Identity(
        pRetVal: *mut *mut IIdentity,
    ) -> HRESULT,

    fn IsInRole(
        role: BSTR, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}
