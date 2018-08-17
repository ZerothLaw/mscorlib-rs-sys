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

RIDL!{#[uuid(0xb4701c26, 0x1509, 0x3726, 0xb2, 0xe1, 0x40, 0x9a, 0x63, 0x6c, 0x9b, 0x4f)]
interface _GenericPrincipal(_GenericPrincipalVtbl): IDispatch(IDispatchVtbl)  
{}} //ClaimsPrincipal