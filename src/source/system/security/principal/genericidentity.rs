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

RIDL!{#[uuid(0x9a37d8b2, 0x2256, 0x3fe3, 0x8b, 0xf0, 0x4f, 0xc4, 0x21, 0xa1, 0x24, 0x4f)]
interface _GenericIdentity(_GenericIdentityVtbl): IDispatch(IDispatchVtbl)  
{}} // ClaimsIdentity