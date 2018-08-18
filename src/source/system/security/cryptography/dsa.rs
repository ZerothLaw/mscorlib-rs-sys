// dsa.rs - MIT License
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
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY};

//struct __declspec(uuid("0c646f46-aa27-350d-88dd-d8c920ce6c2d"))
STRUCT!{struct DSAParameters
{
    P: *mut SAFEARRAY,
    Q: *mut SAFEARRAY,
    G: *mut SAFEARRAY,
    y: *mut SAFEARRAY,
    J: *mut SAFEARRAY,
    x: *mut SAFEARRAY,
    Seed: *mut SAFEARRAY,
    Counter: c_long,
}}

RIDL!{#[uuid(0x0eb5b5e0, 0x1be6, 0x3a5f, 0x87, 0xb3, 0xe3, 0x32, 0x33, 0x42, 0xf4, 0x4e)]
interface _DSA(_DSAVtbl): IDispatch(IDispatchVtbl)  
{}} //AssymetricAlgorithm