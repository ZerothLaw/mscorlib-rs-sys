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

RIDL!{#[uuid(0x9cf4339a, 0x2911, 0x3b8a, 0x8f, 0x30, 0xe5, 0xc6, 0xb5, 0xbe, 0x9a, 0x29)]
interface _StackOverflowException(_StackOverflowExceptionVtbl): IDispatch(IDispatchVtbl)  
{}} //SystemException