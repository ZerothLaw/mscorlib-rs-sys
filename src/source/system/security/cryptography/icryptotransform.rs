// icryptotransform.rs - MIT License
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

use winapi::ctypes::c_long;
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY};

RIDL!{#[uuid(0x8abad867, 0xf515, 0x3cf6, 0xbb, 0x62, 0x5f, 0x0c, 0x88, 0xb3, 0xbb, 0x11)]
interface ICryptoTransform(ICryptoTransformVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_InputBlockSize(
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn get_OutputBlockSize(
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn get_CanTransformMultipleBlocks(
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanReuseTransform(
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn TransformBlock(
        inputBuffer: *mut SAFEARRAY, //byte[]
        inputOffset: c_long,
        inputCount: c_long, 
        outputBuffer: *mut SAFEARRAY, //byte[]
        outputOffset: c_long,
        pRetVal: *mut c_long, 
    ) -> HRESULT,
    fn TransformFinalBlock(
        inputBuffer: *mut SAFEARRAY, //byte[]
        inputOffset: c_long,
        inputCount: c_long,
        pRetVal: *mut *mut SAFEARRAY, //byte[]
    ) -> HRESULT,
}}