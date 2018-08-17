// overlapped.rs - MIT License
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

use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//struct __declspec(uuid("a2959123-2f66-35b4-815d-37c83360809b"))
STRUCT!{struct NativeOverlapped {
    InternalLow: c_long,
    InternalHigh: c_long,
    OffsetLow: c_long,
    OffsetHigh: c_long,
    EventHandle: c_long,
}}

RIDL!{#[uuid(0xdd846fcc, 0x8d04, 0x3665, 0x81, 0xb6, 0xaa, 0xcb, 0xe9, 0x9c, 0x19, 0xc3)]
interface _Overlapped(_OverlappedVtbl): IDispatch(IDispatchVtbl)  
{}} //