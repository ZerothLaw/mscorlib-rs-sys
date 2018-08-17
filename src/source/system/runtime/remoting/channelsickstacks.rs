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
use winapi::um::oaidl::VARIANT;

use system::io::_Stream;
use system::runtime::remoting::ichannel::ITransportHeaders;
use system::iasyncresult::IAsyncResult;
use system::runtime::remoting::ichannel::IServerChannelSink;
use system::iexception::_Exception;
use system::runtime::remoting::ichannel::IClientChannelSink;

use system::runtime::remoting::IMessage;

RIDL!{#[uuid(0x3a5fde6b, 0xdb46, 0x34e8, 0xba, 0xcd, 0x16, 0xea, 0x5a, 0x44, 0x05, 0x40)]
interface IClientChannelSinkStack(IClientChannelSinkStackVtbl): IDispatch(IDispatchVtbl)  
{
    fn Push(
        sink: *mut IClientChannelSink, 
        state: VARIANT, 
    ) -> HRESULT,
    fn Pop(
		sink: *mut  IClientChannelSink ,
		pRetVal: *mut VARIANT ,
	) -> HRESULT,
}}

RIDL!{#[uuid(0x3afab213, 0xf5a2, 0x3241, 0x93, 0xba, 0x32, 0x9e, 0xa4, 0xba, 0x80, 0x16)]
interface IClientResponseChannelSinkStack(IClientResponseChannelSinkStackVtbl): IDispatch(IDispatchVtbl)  
{
    fn AsyncProcessResponse(
        headers: *mut ITransportHeaders, 
        Stream: *mut _Stream, 
    ) -> HRESULT,
    fn DispatchReplyMessage(
        msg: *mut IMessage, 
    ) -> HRESULT,
    fn DispatchException(
        e: *mut _Exception,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xe694a733, 0x768d, 0x314d, 0xb3, 0x17, 0xdc, 0xea, 0xd1, 0x36, 0xb1, 0x1d)]
interface IServerChannelSinkStack(IServerChannelSinkStackVtbl): IDispatch(IDispatchVtbl)  
{
    fn Push(
        sink: IServerChannelSink, 
        state: VARIANT, 
    ) -> HRESULT,
    fn Pop(
		sink: *mut  IServerChannelSink ,
		pRetVal: *mut VARIANT ,
	) -> HRESULT,
    fn Store(
        sink: *mut IServerChannelSink, 
        state: VARIANT, 
    ) -> HRESULT,
    fn StoreAndDispatch(
        sink: *mut IServerChannelSink, 
    ) -> HRESULT,
    fn ServerCallback(
        ar: *mut IAsyncResult,
    ) -> HRESULT,
}} //inherits from IServerResponseChannelSinkStack

RIDL!{#[uuid(0x9be679a6, 0x61fd, 0x38fc, 0xa7, 0xb2, 0x89, 0x98, 0x2d, 0x33, 0x33, 0x8b)]
interface IServerResponseChannelSinkStack(IServerResponseChannelSinkStackVtbl): IDispatch(IDispatchVtbl)  
{
    fn AsyncProcessResponse(
        msg: *mut IMessage,
        headers: *mut ITransportHeaders,
        Stream: *mut _Stream,
    ) -> HRESULT,

    fn GetResponseStream(
        msg: *mut IMessage,
        headers: *mut ITransportHeaders,
        pRetVal: *mut *mut _Stream,
    ) -> HRESULT,
}}