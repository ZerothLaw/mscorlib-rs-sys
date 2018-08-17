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

RIDL!{#[uuid(0x1cac0bda, 0xac58, 0x31bc, 0xb6, 0x24, 0x63, 0xf7, 0x7d, 0x0c, 0x3d, 0x2f)]
interface _MACTripleDES(_MACTripleDESVtbl): IDispatch(IDispatchVtbl)  
{}} //KeyedHashAlgorithm
