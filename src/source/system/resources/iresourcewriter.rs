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
use winapi::shared::wtypes::{BSTR};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY, VARIANT};

RIDL!{#[uuid(0xe97aa6e5, 0x595e, 0x31c3, 0x82, 0xf0, 0x68, 0x8f, 0xb9, 0x19, 0x54, 0xc6)]
interface IResourceWriter(IResourceWriterVtbl) : IDispatch(IDispatchVtbl)
{
    fn AddResource(
        name: BSTR, 
        val: BSTR,
    ) -> HRESULT,
    fn AddResource_2(
        name: BSTR, 
        val: VARIANT,
    ) -> HRESULT,
    fn AddResource_3(
        name: BSTR, 
        val: *mut SAFEARRAY, //byte[]
    ) -> HRESULT,
    fn Close() -> HRESULT, 
    fn Generate() -> HRESULT,
}}