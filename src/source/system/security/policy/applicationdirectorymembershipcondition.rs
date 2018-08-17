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

RIDL!{#[uuid(0xa02a2b22, 0x1dba, 0x3f92, 0x9f, 0x84, 0x55, 0x63, 0x18, 0x28, 0x51, 0xbb)]
interface _ApplicationDirectoryMembershipCondition(_ApplicationDirectoryMembershipConditionVtbl): IDispatch(IDispatchVtbl)  
{}}