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

use system::runtime::remoting::imessage::IMessage;
use system::runtime::remoting::imessagectrl::IMessageCtrl;

RIDL!{#[uuid(0x941f8aaa, 0xa353, 0x3b1d, 0xa0, 0x19, 0x12, 0xe4, 0x43, 0x77, 0xf1, 0xcd)]
interface IMessageSink(IMessageSinkVtbl): IDispatch(IDispatchVtbl)  
{
    fn SyncProcessMessage(
		msg: *mut  IMessage ,
		pRetVal: *mut *mut  IMessage ,
	) -> HRESULT,
    fn AsyncProcessMessage(
        msg: *mut IMessage, 
        replySink: *mut IMessageSink, 
        pRetVal: *mut *mut IMessageCtrl, 
    ) -> HRESULT,
    fn get_NextSink(
		pRetVal: *mut *mut  IMessageSink ,
	) -> HRESULT,
}}