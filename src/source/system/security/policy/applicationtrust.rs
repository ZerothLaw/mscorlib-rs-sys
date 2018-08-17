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

//enum __declspec(uuid("d93eaca8-8176-387b-9667-6d32b504047b"))
ENUM!{enum ApplicationVersionMatch {
    ApplicationVersionMatch_MatchExactVersion = 0,
    ApplicationVersionMatch_MatchAllVersions = 1,
}}

RIDL!{#[uuid(0xe66a9755, 0x58e2, 0x3fcb, 0xa2, 0x65, 0x83, 0x58, 0x51, 0xcb, 0xf0, 0x63)]
interface _ApplicationTrust(_ApplicationTrustVtbl): IDispatch(IDispatchVtbl)  
{}} //EvidenceBase, ISecurityEncodable