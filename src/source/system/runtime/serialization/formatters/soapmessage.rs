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

RIDL!{#[uuid(0xbc0847b2, 0xbd5c, 0x37b3, 0xba, 0x67, 0x7d, 0x2d, 0x54, 0xb1, 0x72, 0x38)]
interface _SoapMessage(_SoapMessageVtbl): IDispatch(IDispatchVtbl)  
{}} //implements ISoapMessage;