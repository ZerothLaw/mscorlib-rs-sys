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

RIDL!{#[uuid(0x010fc1d0, 0x3ef9, 0x3f3b, 0xaa, 0x0a, 0xb7, 0x8a, 0x1f, 0xf8, 0x3a, 0x37)]
interface _UTF8Encoding(_UTF8EncodingVtbl): IDispatch(IDispatchVtbl)  
{}} //Encoding