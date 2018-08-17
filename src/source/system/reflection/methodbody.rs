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

//enum __declspec(uuid("6bd98650-5ae6-3f03-b6cf-1463bbd45e6d"))
ENUM!{enum ExceptionHandlingClauseOptions {
    ExceptionHandlingClauseOptions_Clause = 0,
    ExceptionHandlingClauseOptions_Filter = 1,
    ExceptionHandlingClauseOptions_Finally = 2,
    ExceptionHandlingClauseOptions_Fault = 4,
}}

RIDL!{#[uuid(0x643a4016, 0x1b16, 0x3ccf, 0xae, 0x86, 0x9c, 0x2d, 0x91, 0x35, 0xec, 0xb0)]
interface _ExceptionHandlingClause(_ExceptionHandlingClauseVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xb072efe2, 0xc943, 0x3977, 0xbf, 0xd9, 0x91, 0xd5, 0x23, 0x2b, 0x0d, 0x53)]
interface _MethodBody(_MethodBodyVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf2ecd8ca, 0x91a2, 0x31e8, 0xb8, 0x08, 0xe0, 0x28, 0xb4, 0xf5, 0xca, 0x67)]
interface _LocalVariableInfo(_LocalVariableInfoVtbl): IDispatch(IDispatchVtbl)  
{}}