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

RIDL!{#[uuid(0xfe8a2546, 0x3478, 0x3fad, 0xbe, 0x1d, 0xda, 0x7b, 0xc2, 0x5c, 0x4e, 0x4e)]
interface _CodeConnectAccess(_CodeConnectAccessVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa8f69eca, 0x8c48, 0x3b5e, 0x92, 0xa1, 0x65, 0x49, 0x25, 0x05, 0x80, 0x59)]
interface _NetCodeGroup(_NetCodeGroupVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeGroup, IUnionSemanticCodeGroup