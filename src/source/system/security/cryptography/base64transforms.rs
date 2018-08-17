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

//enum __declspec(uuid("11472518-c3b8-3bf4-9705-2135e1709883"))
ENUM!{enum FromBase64TransformMode
{
    FromBase64TransformMode_IgnoreWhiteSpaces = 0,
    FromBase64TransformMode_DoNotIgnoreWhiteSpaces = 1,
}}

RIDL!{#[uuid(0x23ded1e1, 0x7d5f, 0x3936, 0xaa, 0x4e, 0x18, 0xbb, 0xcc, 0x39, 0xb1, 0x55)]
interface _ToBase64Transform(_ToBase64TransformVtbl): IDispatch(IDispatchVtbl)  
{}} //ICryptoTransform

RIDL!{#[uuid(0xfc0717a6, 0x2e86, 0x372f, 0x81, 0xf4, 0xb3, 0x5e, 0xd4, 0xbd, 0xf0, 0xde)]
interface _FromBase64Transform(_FromBase64TransformVtbl): IDispatch(IDispatchVtbl)  
{}} //ICryptoTransform