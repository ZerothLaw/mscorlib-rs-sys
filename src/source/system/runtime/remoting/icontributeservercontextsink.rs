// icontributeservercontextsink.rs - MIT License
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
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

use system::runtime::remoting::IMessageSink;

RIDL!{#[uuid(0x0caa23ec, 0xf78c, 0x39c9, 0x8d, 0x25, 0xb7, 0xa9, 0xce, 0x40, 0x97, 0xa7)]
interface IContributeServerContextSink(IContributeServerContextSinkVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetServerContextSink(
		NextSink: *mut  IMessageSink ,
		pRetVal: *mut *mut  IMessageSink ,
	) -> HRESULT,
}}