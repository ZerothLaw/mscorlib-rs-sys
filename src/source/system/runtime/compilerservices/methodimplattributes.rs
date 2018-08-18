// methodimplattributes.rs - MIT License
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

//enum __declspec(uuid("63a2e7fd-9a9b-3d6b-a827-3c5bf8db1e6a"))
ENUM!{enum MethodImplOptions
{
    MethodImplOptions_Unmanaged = 4,
    MethodImplOptions_ForwardRef = 16,
    MethodImplOptions_PreserveSig = 128,
    MethodImplOptions_InternalCall = 4096,
    MethodImplOptions_Synchronized = 32,
    MethodImplOptions_NoInlining = 8,
    MethodImplOptions_NoOptimization = 64,
}}

//enum __declspec(uuid("6b7f18ae-f5ac-368f-8dfd-ab5e2d229ed7"))
ENUM!{enum MethodCodeType
{
    MethodCodeType_IL = 0,
    MethodCodeType_Native = 1,
    MethodCodeType_OPTIL = 2,
    MethodCodeType_Runtime = 3,
}}

RIDL!{#[uuid(0x98966503, 0x5d80, 0x3242, 0x83, 0xef, 0x79, 0xe1, 0x36, 0xf6, 0xb9, 0x54)]
interface _MethodImplAttribute(_MethodImplAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}