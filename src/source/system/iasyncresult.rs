// iasyncresult.rs - MIT License
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
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, VARIANT};

use crate::system::threading::_WaitHandle;

RIDL!{#[uuid(0x11ab34e7, 0x0176, 0x3c9e, 0x9e, 0xfe, 0x19, 0x78, 0x58, 0x40, 0x0a, 0x3d)]
interface IAsyncResult(IAsyncResultVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_IsCompleted(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_AsyncWaitHandle(
			pRetVal: *mut *mut _WaitHandle,
	) -> HRESULT,
    fn get_AsyncState(
        pRetVal: *mut *mut VARIANT,
    ) -> HRESULT,
      fn get_CompletedSynchronously(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
}}