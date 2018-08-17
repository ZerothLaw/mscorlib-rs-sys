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

RIDL!{#[uuid(0x18e473f6, 0x637b, 0x3c01, 0x8d, 0x46, 0xd0, 0x11, 0xaa, 0xd2, 0x6c, 0x95)]
interface _ApplicationSecurityInfo(_ApplicationSecurityInfoVtbl): IDispatch(IDispatchVtbl)  
{}}