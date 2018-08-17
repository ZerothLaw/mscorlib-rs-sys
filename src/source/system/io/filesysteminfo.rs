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
RIDL!{#[uuid(0xa5d29a57, 0x36a8, 0x3e36, 0xa0, 0x99, 0x74, 0x58, 0xb1, 0xfa, 0xba, 0xa2)]
interface _FileSystemInfo(_FileSystemInfoVtbl): IDispatch(IDispatchVtbl)  
{}}
