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

use system::runtime::remoting::IMessageSink;

RIDL!{#[uuid(0x4db956b7, 0x69d0, 0x312a, 0xaa, 0x75, 0x44, 0xfb, 0x55, 0xfd, 0x5d, 0x4b)]
interface IContributeClientContextSink(IContributeClientContextSinkVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetClientContextSink(
		NextSink: *mut  IMessageSink ,
		pRetVal: *mut *mut  IMessageSink ,
	) -> HRESULT,
}}