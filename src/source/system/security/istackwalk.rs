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
use winapi::shared::winerror::HRESULT;

RIDL!{#[uuid(0x60fc57b0, 0x4a46, 0x32a0, 0xa5, 0xb4, 0xb0, 0x5b, 0x0d, 0xe8, 0xe7, 0x81)]
interface IStackWalk(IStackWalkVtbl): IDispatch(IDispatchVtbl)  
{
    fn Assert() -> HRESULT,
    fn Demand() -> HRESULT,
    fn Deny() -> HRESULT,
    fn PermitOnly() -> HRESULT,
}}