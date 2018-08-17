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

RIDL!{#[uuid(0x3741bc6f, 0x101b, 0x36d7, 0xa9, 0xd5, 0x03, 0xfc, 0xc0, 0xec, 0xda, 0x35)]
interface _TimerCallback(_TimerCallbackVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xb49a029b, 0x406b, 0x3b1e, 0x88, 0xe4, 0xf8, 0x66, 0x90, 0xd2, 0x03, 0x64)]
interface _Timer(_TimerVtbl): IDispatch(IDispatchVtbl)  
{}}