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

RIDL!{#[uuid(0x0d9f1b65, 0x6d27, 0x3e9f, 0xba, 0xf3, 0x05, 0x97, 0x83, 0x7e, 0x0f, 0x33)]
interface _DBNull(_DBNullVtbl): IDispatch(IDispatchVtbl)  
{}} //ISerializable, IConvertible
