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

RIDL!{#[uuid(0x53bce4d4, 0x6209, 0x396d, 0xbd, 0x4a, 0x0b, 0x0a, 0x0a, 0x17, 0x7d, 0xf9)]
interface _CallContext(_CallContextVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x4d125449, 0xba27, 0x3927, 0x85, 0x89, 0x3e, 0x1b, 0x34, 0xb6, 0x22, 0xe5)]
interface ILogicalThreadAffinative(ILogicalThreadAffinativeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x9aff21f5, 0x1c9c, 0x35e7, 0xae, 0xa4, 0xc3, 0xaa, 0x0b, 0xeb, 0x3b, 0x77)]
interface _LogicalCallContext(_LogicalCallContextVtbl): IDispatch(IDispatchVtbl)  
{}} //ISerializable, ICloneable

