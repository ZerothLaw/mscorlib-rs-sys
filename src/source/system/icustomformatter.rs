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

use winapi::um::oaidl::{IDispatch, IDispatchVtbl, VARIANT};

use system::IFormatProvider;

RIDL!{#[uuid(0x2b130940, 0xca5e, 0x3406, 0x83, 0x85, 0xe2, 0x59, 0xe6, 0x8a, 0xb0, 0x39)]
interface ICustomFormatter(ICustomFormatterVtbl): IDispatch(IDispatchVtbl)  
{
    fn format(
        format: BSTR, 
        arg: *mut VARIANT, 
        formatProvider: *mut IFormatProvider,
        pRetVal: *mut BSTR,
    ) -> HRESULT,
}}