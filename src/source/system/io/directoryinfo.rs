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
RIDL!{#[uuid(0x487e52f1, 0x2bb9, 0x3bd0, 0xa0, 0xca, 0x67, 0x28, 0xb3, 0xa1, 0xd0, 0x51)]
interface _DirectoryInfo(_DirectoryInfoVtbl): IDispatch(IDispatchVtbl)  
{}} //FileSystemInfo
