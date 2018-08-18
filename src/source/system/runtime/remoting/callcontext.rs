// callcontext.rs - MIT License
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

RIDL!{#[uuid(0x53bce4d4, 0x6209, 0x396d, 0xbd, 0x4a, 0x0b, 0x0a, 0x0a, 0x17, 0x7d, 0xf9)]
interface _CallContext(_CallContextVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x4d125449, 0xba27, 0x3927, 0x85, 0x89, 0x3e, 0x1b, 0x34, 0xb6, 0x22, 0xe5)]
interface ILogicalThreadAffinative(ILogicalThreadAffinativeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x9aff21f5, 0x1c9c, 0x35e7, 0xae, 0xa4, 0xc3, 0xaa, 0x0b, 0xeb, 0x3b, 0x77)]
interface _LogicalCallContext(_LogicalCallContextVtbl): IDispatch(IDispatchVtbl)  
{}} //ISerializable, ICloneable

