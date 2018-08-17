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

//enum __declspec(uuid("ee965595-853a-331b-9cd0-d53dcce3b6f8"))
ENUM!{enum PolicyLevelType
{
    PolicyLevelType_User = 0,
    PolicyLevelType_Machine = 1,
    PolicyLevelType_Enterprise = 2,
    PolicyLevelType_AppDomain = 3,
}}

RIDL!{#[uuid(0xabc04b16, 0x5539, 0x3c7e, 0x92, 0xec, 0x09, 0x05, 0xa4, 0xa2, 0x44, 0x64)]
interface _SecurityManager(_SecurityManagerVtbl): IDispatch(IDispatchVtbl)  
{}} //static?