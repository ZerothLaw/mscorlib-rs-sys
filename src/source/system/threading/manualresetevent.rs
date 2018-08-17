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

RIDL!{#[uuid(0xc0bb9361, 0x268f, 0x3e72, 0xbf, 0x6f, 0x41, 0x20, 0x17, 0x5a, 0x15, 0x00)]
interface _ManualResetEvent(_ManualResetEventVtbl): IDispatch(IDispatchVtbl)  
{}} //EventWaitHandle
