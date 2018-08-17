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
use winapi::shared::ntdef::{HRESULT};

use winapi::um::oaidl::{VARIANT};

RIDL!{#[uuid(0xab3f47e4, 0xc227, 0x3b05, 0xbf, 0x9f, 0x94, 0x64, 0x9b, 0xef, 0x98, 0x88)]
interface IDeserializationCallback(IDeserializationCallbackVtbl) : IDispatch(IDispatchVtbl)
{
    fn OnDeserialization(
        sender: VARIANT, 
    ) -> HRESULT,
}}