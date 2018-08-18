// rsa.rs - MIT License
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

use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY};

//struct __declspec(uuid("094e9135-483d-334a-aae7-8690895ab70a"))
STRUCT!{struct RSAParameters
{
    Exponent: *mut SAFEARRAY, //all byte[]
    Modulus: *mut SAFEARRAY,
    P: *mut SAFEARRAY,
    Q: *mut SAFEARRAY,
    DP: *mut SAFEARRAY,
    DQ: *mut SAFEARRAY,
    InverseQ: *mut SAFEARRAY,
    D: *mut SAFEARRAY,
}}

RIDL!{#[uuid(0x0b3fb710, 0xa25c, 0x3310, 0x87, 0x74, 0x1c, 0xf1, 0x17, 0xf9, 0x5b, 0xd4)]
interface _RSA(_RSAVtbl): IDispatch(IDispatchVtbl)  
{}} //AsymmetricAlgorithm