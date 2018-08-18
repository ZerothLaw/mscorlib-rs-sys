// iresourcewriter.rs - MIT License
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
use winapi::shared::wtypes::{BSTR};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY, VARIANT};

RIDL!{#[uuid(0xe97aa6e5, 0x595e, 0x31c3, 0x82, 0xf0, 0x68, 0x8f, 0xb9, 0x19, 0x54, 0xc6)]
interface IResourceWriter(IResourceWriterVtbl) : IDispatch(IDispatchVtbl)
{
    fn AddResource(
        name: BSTR, 
        val: BSTR,
    ) -> HRESULT,
    fn AddResource_2(
        name: BSTR, 
        val: VARIANT,
    ) -> HRESULT,
    fn AddResource_3(
        name: BSTR, 
        val: *mut SAFEARRAY, //byte[]
    ) -> HRESULT,
    fn Close() -> HRESULT, 
    fn Generate() -> HRESULT,
}}