// threadpool.rs - MIT License
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

RIDL!{#[uuid(0x64409425, 0xf8c9, 0x370e, 0x80, 0x9e, 0x32, 0x41, 0xce, 0x80, 0x45, 0x41)]
interface _RegisteredWaitHandle(_RegisteredWaitHandleVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xce949142, 0x4d4c, 0x358d, 0x89, 0xa9, 0xe6, 0x9a, 0x53, 0x1a, 0xa3, 0x63)]
interface _WaitCallback(_WaitCallbackVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf078f795, 0xf452, 0x3d2d, 0x8c, 0xc8, 0x16, 0xd6, 0x6a, 0xe4, 0x6c, 0x67)]
interface _WaitOrTimerCallback(_WaitOrTimerCallbackVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xbbae942d, 0xbff4, 0x36e2, 0xa3, 0xbc, 0x50, 0x8b, 0xb3, 0x80, 0x1f, 0x4f)]
interface _IOCompletionCallback(_IOCompletionCallbackVtbl): IDispatch(IDispatchVtbl)  
{}}