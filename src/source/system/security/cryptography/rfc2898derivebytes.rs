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

RIDL!{#[uuid(0xa6589897, 0x5a67, 0x305f, 0x94, 0x97, 0x72, 0xe5, 0xfe, 0x8b, 0xea, 0xd5)]
interface _Rfc2898DeriveBytes(_Rfc2898DeriveBytesVtbl): IDispatch(IDispatchVtbl)  
{}} //DeriveBytes
