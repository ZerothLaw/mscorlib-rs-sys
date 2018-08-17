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

//enum __declspec(uuid("8990cb3b-227e-3a43-8264-0057ec763fa0"))
ENUM!{enum CryptoStreamMode
{
    CryptoStreamMode_Read = 0,
    CryptoStreamMode_Write = 1,
}}

RIDL!{#[uuid(0x4134f762, 0xd0ec, 0x3210, 0x93, 0xc0, 0xde, 0x4f, 0x44, 0x3d, 0x56, 0x69)]
interface _CryptoStream(_CryptoStreamVtbl): IDispatch(IDispatchVtbl)  
{}} //Stream, IDisposable