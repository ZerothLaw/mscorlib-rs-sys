// crypto.rs - MIT License
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

use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("75c9e85e-d2d1-32db-bf9c-0636f94fb0c2"))
ENUM!{enum CipherMode
{
    CipherMode_CBC = 1,
    CipherMode_ECB = 2,
    CipherMode_OFB = 3,
    CipherMode_CFB = 4,
    CipherMode_CTS = 5,
}}

//enum __declspec(uuid("1254089d-0104-3bfb-b6ba-9168f994dca6"))
ENUM!{enum PaddingMode
{
    PaddingMode_None = 1,
    PaddingMode_PKCS7 = 2,
    PaddingMode_Zeros = 3,
    PaddingMode_ANSIX923 = 4,
    PaddingMode_ISO10126 = 5,
}}

RIDL!{#[uuid(0x8978b0be, 0xa89e, 0x3ff9, 0x98, 0x34, 0x77, 0x86, 0x2c, 0xeb, 0xff, 0x3d)]
interface _KeySizes(_KeySizesVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x4311e8f5, 0xb249, 0x3f81, 0x8f, 0xf4, 0xcf, 0x85, 0x3d, 0x85, 0x30, 0x6d)]
interface _CryptographicException(_CryptographicExceptionVtbl): IDispatch(IDispatchVtbl)  
{}} //SystemException

RIDL!{#[uuid(0x7fb08423, 0x038f, 0x3acc, 0xb6, 0x00, 0xe6, 0xd0, 0x72, 0xba, 0xe1, 0x60)]
interface _CryptographicUnexpectedOperationException(_CryptographicUnexpectedOperationExceptionVtbl): IDispatch(IDispatchVtbl)  
{}} //CryptographicException
