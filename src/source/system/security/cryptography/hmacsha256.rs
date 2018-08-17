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

RIDL!{#[uuid(0x1377ce34, 0x8921, 0x3bd4, 0x96, 0xe9, 0xc8, 0xd5, 0xd5, 0xaa, 0x1a, 0xdf)]
interface _HMACSHA256(_HMACSHA256Vtbl): IDispatch(IDispatchVtbl)  
{}} //hmac