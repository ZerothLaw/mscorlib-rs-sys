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

use system::collections::IDictionary;

RIDL!{#[uuid(0x1a8b0de6, 0xb825, 0x38c5, 0xb7, 0x44, 0x8f, 0x93, 0x07, 0x5f, 0xd6, 0xfa)]
interface IMessage(IMessageVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_Properties(
		pRetVal: *mut *mut  IDictionary ,
	) -> HRESULT,
}}