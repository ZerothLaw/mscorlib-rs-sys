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

RIDL!{#[uuid(0x5767c78f, 0xf344, 0x35a5, 0x84, 0xbc, 0x53, 0xb9, 0xea, 0xeb, 0x68, 0xcb)]
interface _RijndaelManagedTransform(_RijndaelManagedTransformVtbl): IDispatch(IDispatchVtbl)  
{}} //ICryptoTransform
