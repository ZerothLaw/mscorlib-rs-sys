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

RIDL!{#[uuid(0xa9c9f3d9, 0xe153, 0x39b8, 0xa5, 0x33, 0xb8, 0xdf, 0x46, 0x64, 0x40, 0x7b)]
interface _PolicyException(_PolicyExceptionVtbl): IDispatch(IDispatchVtbl)  
{}} //System.Exception