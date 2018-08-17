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

RIDL!{#[uuid(0x68d5592b, 0x47c8, 0x381a, 0x8d, 0x51, 0x39, 0x25, 0xc1, 0x6c, 0xf0, 0x25)]
interface _IsolatedStorageFileStream(_IsolatedStorageFileStreamVtbl): IDispatch(IDispatchVtbl)  
{}} //_FileStream