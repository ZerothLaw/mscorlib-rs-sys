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

RIDL!{#[uuid(0xf7c0c4cc, 0x0d49, 0x31ee, 0xa3, 0xd3, 0xb8, 0xb5, 0x51, 0xe4, 0x92, 0x8c)]
interface _RC2(_RC2Vtbl): IDispatch(IDispatchVtbl)  
{}} //SymmetricAlgorithm