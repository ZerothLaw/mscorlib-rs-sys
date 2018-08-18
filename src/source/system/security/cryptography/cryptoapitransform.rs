// cryptoapitransform.rs - MIT License
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

RIDL!{#[uuid(0x983b8639, 0x2ed7, 0x364c, 0x98, 0x99, 0x68, 0x2a, 0xbb, 0x2c, 0xe8, 0x50)]
interface _CryptoAPITransform(_CryptoAPITransformVtbl): IDispatch(IDispatchVtbl)  
{}} //ICryptoTransform

//enum __declspec(uuid("6be41cdf-29d7-32db-8181-5117f580ba68"))
ENUM!{enum CspProviderFlags
{
    CspProviderFlags_NoFlags = 0,
    CspProviderFlags_UseMachineKeyStore = 1,
    CspProviderFlags_UseDefaultKeyContainer = 2,
    CspProviderFlags_UseNonExportableKey = 4,
    CspProviderFlags_UseExistingKey = 8,
    CspProviderFlags_UseArchivableKey = 16,
    CspProviderFlags_UseUserProtectedKey = 32,
    CspProviderFlags_NoPrompt = 64,
    CspProviderFlags_CreateEphemeralKey = 128,
}}

RIDL!{#[uuid(0xd5331d95, 0xfff2, 0x358f, 0xaf, 0xd5, 0x58, 0x8f, 0x46, 0x9f, 0xf2, 0xe4)]
interface _CspParameters(_CspParametersVtbl): IDispatch(IDispatchVtbl)  
{}}