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

//enum __declspec(uuid("fdbf0369-d278-3320-b9ce-0e0719380c0f"))
ENUM!{enum CompareOptions
{
    CompareOptions_None = 0,
    CompareOptions_IgnoreCase = 1,
    CompareOptions_IgnoreNonSpace = 2,
    CompareOptions_IgnoreSymbols = 4,
    CompareOptions_IgnoreKanaType = 8,
    CompareOptions_IgnoreWidth = 16,
    CompareOptions_OrdinalIgnoreCase = 268435456,
    CompareOptions_StringSort = 536870912,
    CompareOptions_Ordinal = 1073741824,
}}

RIDL!{#[uuid(0x505defe5, 0xaefa, 0x3e23, 0x82, 0xb0, 0xd5, 0xeb, 0x08, 0x5b, 0xb8, 0x40)]
interface _CompareInfo(_CompareInfoVtbl): IDispatch(IDispatchVtbl)  
{}}
