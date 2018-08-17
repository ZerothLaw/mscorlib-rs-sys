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

RIDL!{#[uuid(0x3bcf0cb2, 0xa849, 0x375e, 0x81, 0x89, 0x1b, 0xa5, 0xf1, 0xf4, 0xa9, 0xb0)]
interface _BinaryFormatter(_BinaryFormatterVtbl): IDispatch(IDispatchVtbl)  
{}} //implements IRemotingFormatter/IFormatter