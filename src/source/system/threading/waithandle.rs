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

RIDL!{#[uuid(0x40dfc50a, 0xe93a, 0x3c08, 0xb9, 0xef, 0xe2, 0xb4, 0xf2, 0x8b, 0x56, 0x76)]
interface _WaitHandle(_WaitHandleVtbl): IDispatch(IDispatchVtbl)  
{}}// MarshalByRefObject, IDisposable