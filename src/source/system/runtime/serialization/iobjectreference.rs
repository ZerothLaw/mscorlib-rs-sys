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
use winapi::um::oaidl::VARIANT;
use winapi::shared::winerror::HRESULT;

use system::runtime::serialization::streamingcontext::StreamingContext;

RIDL!{#[uuid(0x6e70ed5f, 0x0439, 0x38ce, 0x83, 0xbb, 0x86, 0x0f, 0x14, 0x21, 0xf2, 0x9f)]
interface IObjectReference(IObjectReferenceVtbl) : IDispatch(IDispatchVtbl) {
    fn GetRealObject(
        Context: StreamingContext, 
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
}}
