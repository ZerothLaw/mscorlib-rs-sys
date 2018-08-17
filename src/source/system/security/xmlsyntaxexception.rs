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

RIDL!{#[uuid(0xd9fcad88, 0xd869, 0x3788, 0xa8, 0x02, 0x1b, 0x1e, 0x00, 0x7c, 0x7a, 0x22)]
interface _XmlSyntaxException(_XmlSyntaxExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}