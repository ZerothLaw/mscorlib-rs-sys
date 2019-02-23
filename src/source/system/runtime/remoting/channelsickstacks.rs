// channelsickstacks.rs - MIT License
//  MIT License
//  Copyright (c) 2018 Tyler Laing (ZerothLaw)
// 
//  Permission is hereby granted, free of charge, to any person obtaining a copy
//  of this software and associated documentation files (the "Software"), to deal
//  in the Software without restriction, including without limitation the rights
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//  copies of the Software, and to permit persons to whom the Software is
//  furnished to do so, subject to the following conditions:
// 
//  The above copyright notice and this permission notice shall be included in all
//  copies or substantial portions of the Software.
// 
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//  SOFTWARE.

use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, VARIANT};

use crate::system::IAsyncResult;
use crate::system::iexception::_Exception;
use crate::system::io::_Stream;

use crate::system::runtime::remoting::{IClientChannelSink,IMessage,IServerChannelSink,ITransportHeaders};

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