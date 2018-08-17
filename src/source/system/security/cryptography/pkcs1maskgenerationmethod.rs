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

RIDL!{#[uuid(0x425bff0d, 0x59e4, 0x36a8, 0xb1, 0xff, 0x1f, 0x5d, 0x39, 0xd6, 0x98, 0xf4)]
interface _PKCS1MaskGenerationMethod(_PKCS1MaskGenerationMethodVtbl): IDispatch(IDispatchVtbl)  
{}} //MaskGenerationMethod