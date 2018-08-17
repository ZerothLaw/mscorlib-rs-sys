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

RIDL!{#[uuid(0x9fb09782, 0x8d39, 0x3b0c, 0xb7, 0x9e, 0xf7, 0xa3, 0x7a, 0x65, 0xb3, 0xda)]
interface _StringBuilder(_StringBuilderVtbl): IDispatch(IDispatchVtbl)  
{}} //ISerializable