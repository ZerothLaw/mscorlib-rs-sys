// icustomformatter.rs - MIT License
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
use winapi::shared::wtypes::BSTR;

use winapi::um::oaidl::{IDispatch, IDispatchVtbl, VARIANT};

use system::IFormatProvider;

RIDL!{#[uuid(0x2b130940, 0xca5e, 0x3406, 0x83, 0x85, 0xe2, 0x59, 0xe6, 0x8a, 0xb0, 0x39)]
interface ICustomFormatter(ICustomFormatterVtbl): IDispatch(IDispatchVtbl)  
{
    fn format(
        format: BSTR, 
        arg: *mut VARIANT, 
        formatProvider: *mut IFormatProvider,
        pRetVal: *mut BSTR,
    ) -> HRESULT,
}}