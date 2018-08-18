// base64transforms.rs - MIT License
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

//enum __declspec(uuid("11472518-c3b8-3bf4-9705-2135e1709883"))
ENUM!{enum FromBase64TransformMode
{
    FromBase64TransformMode_IgnoreWhiteSpaces = 0,
    FromBase64TransformMode_DoNotIgnoreWhiteSpaces = 1,
}}

RIDL!{#[uuid(0x23ded1e1, 0x7d5f, 0x3936, 0xaa, 0x4e, 0x18, 0xbb, 0xcc, 0x39, 0xb1, 0x55)]
interface _ToBase64Transform(_ToBase64TransformVtbl): IDispatch(IDispatchVtbl)  
{}} //ICryptoTransform

RIDL!{#[uuid(0xfc0717a6, 0x2e86, 0x372f, 0x81, 0xf4, 0xb3, 0x5e, 0xd4, 0xbd, 0xf0, 0xde)]
interface _FromBase64Transform(_FromBase64TransformVtbl): IDispatch(IDispatchVtbl)  
{}} //ICryptoTransform