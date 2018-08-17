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

RIDL!{#[uuid(0x4ca8147e, 0xbaa3, 0x3a7f, 0x92, 0xce, 0xa4, 0xfd, 0x7f, 0x17, 0xd8, 0xda)]
interface _BinaryWriter(_BinaryWriterVtbl): IDispatch(IDispatchVtbl)  
{}} //IDisposable
