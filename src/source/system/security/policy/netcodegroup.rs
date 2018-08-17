// netcodegroup.rs - MIT License
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

RIDL!{#[uuid(0xfe8a2546, 0x3478, 0x3fad, 0xbe, 0x1d, 0xda, 0x7b, 0xc2, 0x5c, 0x4e, 0x4e)]
interface _CodeConnectAccess(_CodeConnectAccessVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa8f69eca, 0x8c48, 0x3b5e, 0x92, 0xa1, 0x65, 0x49, 0x25, 0x05, 0x80, 0x59)]
interface _NetCodeGroup(_NetCodeGroupVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeGroup, IUnionSemanticCodeGroup