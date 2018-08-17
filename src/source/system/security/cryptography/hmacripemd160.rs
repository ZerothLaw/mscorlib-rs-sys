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


RIDL!{#[uuid(0x9fd974a5, 0x338c, 0x37b9, 0xa1, 0xb2, 0xd4, 0x5f, 0x0c, 0x2b, 0x25, 0xc2)]
interface _HMACRIPEMD160(_HMACRIPEMD160Vtbl): IDispatch(IDispatchVtbl)  
{}} //hmac