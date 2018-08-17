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
RIDL!{#[uuid(0xf9e97e04, 0x4e1e, 0x368f, 0xb6, 0xc6, 0x5e, 0x96, 0xce, 0x43, 0x62, 0xd6)]
interface _RegionInfo(_RegionInfoVtbl): IDispatch(IDispatchVtbl)  
{}}