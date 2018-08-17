// runtimehandle.rs - MIT License
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

use winapi::um::unknwnbase::{IUnknown};

//struct __declspec(uuid("f8fc5d7c-8215-3e65-befb-11e8172606fe"))
STRUCT!{struct RuntimeMethodHandle {
    m_value: *mut IUnknown,
}}

//struct __declspec(uuid("27b33bd9-e6f7-3148-911d-f67340a5353f"))
STRUCT!{struct RuntimeFieldHandle {
    m_ptr: *mut IUnknown,
}}

//struct __declspec(uuid("8531f85a-746b-3db5-a45f-9bac4bd02d8b"))
STRUCT!{struct ModuleHandle {
    m_ptr: *mut IUnknown,
}}