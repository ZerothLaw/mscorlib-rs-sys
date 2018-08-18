// isolatedstorage.rs - MIT License
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

//enum __declspec(uuid("b3e5a7ff-afc6-3f2b-8fff-300c7c567693"))
ENUM!{enum IsolatedStorageScope
{
    IsolatedStorageScope_None = 0,
    IsolatedStorageScope_User = 1,
    IsolatedStorageScope_Domain = 2,
    IsolatedStorageScope_Assembly = 4,
    IsolatedStorageScope_Roaming = 8,
    IsolatedStorageScope_Machine = 16,
    IsolatedStorageScope_Application = 32,
}}

RIDL!{#[uuid(0x34ec3bd7, 0xf2f6, 0x3c20, 0xa6, 0x39, 0x80, 0x4b, 0xff, 0x89, 0xdf, 0x65)]
interface _IsolatedStorage(_IsolatedStorageVtbl): IDispatch(IDispatchVtbl)  
{}} //MarshalByRefObject