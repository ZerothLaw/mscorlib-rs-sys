// appdomain.rs - MIT License
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

RIDL!{#[uuid(0x98947cf0, 0x77e7, 0x328e, 0xb7, 0x09, 0x5d, 0xd1, 0xaa, 0x1c, 0x9c, 0x96)]
interface _ResolveEventArgs(_ResolveEventArgsVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7a0325f0, 0x22c2, 0x31f9, 0x88, 0x23, 0x9b, 0x8a, 0xee, 0x94, 0x56, 0xb1)]
interface _AssemblyLoadEventArgs(_AssemblyLoadEventArgsVtbl): IDispatch(IDispatchVtbl)  
{}}


RIDL!{#[uuid(0x8e54a9cc, 0x7aa4, 0x34ca, 0x98, 0x5b, 0xbd, 0x7d, 0x75, 0x27, 0xb1, 0x10)]
interface _ResolveEventHandler(_ResolveEventHandlerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xdeece11f, 0xa893, 0x3e35, 0xa4, 0xc3, 0xda, 0xb7, 0xfa, 0x09, 0x11, 0xeb)]
interface _AssemblyLoadEventHandler(_AssemblyLoadEventHandlerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x5e6f9edb, 0x3ce1, 0x3a56, 0x86, 0xd9, 0xcd, 0x2d, 0xdf, 0x7a, 0x6f, 0xff)]
interface _AppDomainInitializer(_AppDomainInitializerVtbl): IDispatch(IDispatchVtbl)  
{}}

pub use crate::system::iappdomain::_AppDomain;

RIDL!{#[uuid(0xaf93163f, 0xc2f4, 0x3fab, 0x9f, 0xf1, 0x72, 0x8a, 0x7a, 0xaa, 0xd1, 0xcb)]
interface _CrossAppDomainDelegate(_CrossAppDomainDelegateVtbl): IDispatch(IDispatchVtbl)  
{}}