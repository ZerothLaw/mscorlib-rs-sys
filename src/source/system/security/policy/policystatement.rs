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

//enum __declspec(uuid("338d2529-b3d6-37f1-bb01-404698dc537b"))
ENUM!{enum PolicyStatementAttribute
{
    PolicyStatementAttribute_Nothing = 0,
    PolicyStatementAttribute_Exclusive = 1,
    PolicyStatementAttribute_LevelFinal = 2,
    PolicyStatementAttribute_All = 3,
}}

RIDL!{#[uuid(0x3eefd1fc, 0x4d8d, 0x3177, 0x99, 0xf6, 0x6c, 0x19, 0xd9, 0xe0, 0x88, 0xd3)]
interface _PolicyStatement(_PolicyStatementVtbl): IDispatch(IDispatchVtbl)  
{}} // ISecurityPolicyEncodable, ISecurityEncodable