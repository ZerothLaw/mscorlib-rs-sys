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

use system::iformatprovider::IFormatProvider;

RIDL!{#[uuid(0x9a604ee7, 0xe630, 0x3ded, 0x94, 0x44, 0xba, 0xae, 0x24, 0x70, 0x75, 0xab)]
interface IFormattable(IFormattableVtbl): IDispatch(IDispatchVtbl)  
{
    fn ToString(
        format: BSTR, 
        formatProvider: *mut IFormatProvider, 
        pRetVal: *mut BSTR, 
    ) -> HRESULT,
}}
