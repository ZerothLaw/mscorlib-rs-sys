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
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY};

use system::reflection::_Type;

RIDL!{#[uuid(0xb9b91146, 0xd6c2, 0x3a62, 0x81, 0x59, 0xc2, 0xd1, 0x79, 0x4c, 0xde, 0xb0)]
interface ICustomAttributeProvider(ICustomAttributeProviderVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetCustomAttributes(
        attributeType: *mut _Type,
        inherit: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetCustomAttributes_2(
        inherit: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn IsDefined(
        attributeType: *mut _Type,
        inherit: VARIANT_BOOL,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}
