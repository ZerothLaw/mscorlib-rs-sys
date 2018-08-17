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

RIDL!{#[uuid(0x875715c5, 0xcb64, 0x3920, 0x81, 0x56, 0x0e, 0xe9, 0xcb, 0x0e, 0x07, 0xea)]
interface _RC2CryptoServiceProvider(_RC2CryptoServiceProviderVtbl): IDispatch(IDispatchVtbl)  
{}} //RC2