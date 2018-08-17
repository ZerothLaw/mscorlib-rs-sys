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

RIDL!{#[uuid(0x6bbb7dee, 0x186f, 0x3d51, 0x94, 0x86, 0xbe, 0x0a, 0x71, 0xe9, 0x15, 0xce)]
interface _IsolatedStorageFile(_IsolatedStorageFileVtbl): IDispatch(IDispatchVtbl)  
{}} //IsolatedStorage, IDisposable
