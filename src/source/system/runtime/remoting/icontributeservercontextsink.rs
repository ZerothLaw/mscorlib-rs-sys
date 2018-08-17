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

RIDL!{#[uuid(0x0caa23ec, 0xf78c, 0x39c9, 0x8d, 0x25, 0xb7, 0xa9, 0xce, 0x40, 0x97, 0xa7)]
interface IContributeServerContextSink(IContributeServerContextSinkVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetServerContextSink(
		NextSink: *mut  IMessageSink ,
		pRetVal: *mut *mut  IMessageSink ,
	) -> HRESULT,
}}