// attributes.rs - MIT License
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

RIDL!{#[uuid(0x8000e51a, 0x541c, 0x3b20, 0xa8, 0xec, 0xc8, 0xa8, 0xb4, 0x11, 0x16, 0xc4)]
interface _SuppressUnmanagedCodeSecurityAttribute(_SuppressUnmanagedCodeSecurityAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x41f41c1b, 0x7b8d, 0x39a3, 0xa2, 0x8f, 0xaa, 0xe2, 0x07, 0x87, 0xf4, 0x69)]
interface _UnverifiableCodeAttribute(_UnverifiableCodeAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf1c930c4, 0x2233, 0x3924, 0x98, 0x40, 0x23, 0x1d, 0x00, 0x82, 0x59, 0xb4)]
interface _AllowPartiallyTrustedCallersAttribute(_AllowPartiallyTrustedCallersAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}