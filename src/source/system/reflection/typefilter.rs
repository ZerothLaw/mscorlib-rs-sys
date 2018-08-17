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

RIDL!{#[uuid(0xe1817846, 0x3745, 0x3c97, 0xb4, 0xa6, 0xee, 0x20, 0xa1, 0x64, 0x1b, 0x29)]
interface _TypeFilter(_TypeFilterVtbl): IDispatch(IDispatchVtbl)  
{}} //public delegate bool TypeFilter(Type m, Object filterCriteria);