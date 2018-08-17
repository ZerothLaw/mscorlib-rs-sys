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

RIDL!{#[uuid(0x34b0417e, 0xe71d, 0x304c, 0x9f, 0xac, 0x68, 0x93, 0x50, 0xa1, 0xb4, 0x1c)]
interface _PermissionRequestEvidence(_PermissionRequestEvidenceVtbl): IDispatch(IDispatchVtbl)  
{}} //EvidenceBase
//obsolete