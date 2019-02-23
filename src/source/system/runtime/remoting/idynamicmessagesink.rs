// idynamicmessagesink.rs - MIT License
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
use winapi::shared::wtypes::{BSTR,VARIANT_BOOL};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

use crate::system::runtime::remoting::IMessage;

RIDL!{#[uuid(0x00a358d4, 0x4d58, 0x3b9d, 0x8f, 0xb6, 0xfb, 0x7f, 0x6b, 0xc1, 0x71, 0x3b)]
interface IDynamicProperty(IDynamicPropertyVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_name(
        pRetVal: BSTR ,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xc74076bb, 0x8a2d, 0x3c20, 0xa5, 0x42, 0x62, 0x53, 0x29, 0xe9, 0xaf, 0x04)]
interface IDynamicMessageSink(IDynamicMessageSinkVtbl): IDispatch(IDispatchVtbl)  
{
    fn ProcessMessageStart(
        reqMsg: *mut IMessage,
        bCliSide: VARIANT_BOOL,
        bAsync: VARIANT_BOOL,
    ) -> HRESULT,
    fn ProcessMessageFinish(
        reqMsg: *mut IMessage,
        bCliSide: VARIANT_BOOL,
        bAsync: VARIANT_BOOL,
    ) -> HRESULT,
}}