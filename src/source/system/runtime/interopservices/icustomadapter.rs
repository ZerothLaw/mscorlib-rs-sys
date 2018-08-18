// icustomadapter.rs - MIT License
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

use winapi::shared::ntdef::{HRESULT};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::um::unknwnbase::{IUnknown};

RIDL!{#[uuid(0x3cc86595, 0xfeb5, 0x3ce9, 0xba, 0x14, 0xd0, 0x5c, 0x8d, 0xc3, 0x32, 0x1c)]
interface ICustomAdapter(ICustomAdapterVtbl) : IDispatch(IDispatchVtbl)
{
    fn GetUnderlyingObject(
        pRetVal: *mut *mut IUnknown,
    ) -> HRESULT,
}}
