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

//enum __declspec(uuid("63a2e7fd-9a9b-3d6b-a827-3c5bf8db1e6a"))
ENUM!{enum MethodImplOptions
{
    MethodImplOptions_Unmanaged = 4,
    MethodImplOptions_ForwardRef = 16,
    MethodImplOptions_PreserveSig = 128,
    MethodImplOptions_InternalCall = 4096,
    MethodImplOptions_Synchronized = 32,
    MethodImplOptions_NoInlining = 8,
    MethodImplOptions_NoOptimization = 64,
}}

//enum __declspec(uuid("6b7f18ae-f5ac-368f-8dfd-ab5e2d229ed7"))
ENUM!{enum MethodCodeType
{
    MethodCodeType_IL = 0,
    MethodCodeType_Native = 1,
    MethodCodeType_OPTIL = 2,
    MethodCodeType_Runtime = 3,
}}

RIDL!{#[uuid(0x98966503, 0x5d80, 0x3242, 0x83, 0xef, 0x79, 0xe1, 0x36, 0xf6, 0xb9, 0x54)]
interface _MethodImplAttribute(_MethodImplAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}