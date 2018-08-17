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

use winapi::um::oaidl::IDispatchVtbl;
use winapi::um::oaidl::IDispatch;

pub struct UnhandledExceptionEventArgs{}//EventArgs?
RIDL!{#[uuid(0xa218e20a, 0x0905, 0x3741, 0xb0, 0xb3, 0x9e, 0x31, 0x93, 0x16, 0x2e, 0x50)]
interface _UnhandledExceptionEventArgs(_UnhandledExceptionEventArgsVtbl): IDispatch(IDispatchVtbl)  
{}}