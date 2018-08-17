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

RIDL!{#[uuid(0x89b9f00b, 0xaa2a, 0x3a49, 0x91, 0xb4, 0xe8, 0xd1, 0xf1, 0xc0, 0x0e, 0x58)]
interface _UTF7Encoding(_UTF7EncodingVtbl): IDispatch(IDispatchVtbl)  
{}} //Encoding