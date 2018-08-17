// MIT License
// Copyright (c) 2018 Tyler Laing (ZerothLaw)

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x3344e8b4, 0xa5c3, 0x3882, 0x8d, 0x30, 0x63, 0x79, 0x24, 0x85, 0xec, 0xcf)]
interface _DebuggerStepThroughAttribute(_DebuggerStepThroughAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xb3276180, 0xb23e, 0x3034, 0xb1, 0x8f, 0xe0, 0x12, 0x2b, 0xa4, 0xe4, 0xcf)]
interface _DebuggerStepperBoundaryAttribute(_DebuggerStepperBoundaryAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x55b6903b, 0x55fe, 0x35e0, 0x80, 0x4f, 0xe4, 0x2a, 0x09, 0x6d, 0x2e, 0xb0)]
interface _DebuggerHiddenAttribute(_DebuggerHiddenAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xcc6dcafd, 0x0185, 0x308a, 0x89, 0x1c, 0x83, 0x81, 0x2f, 0xe5, 0x74, 0xe7)]
interface _DebuggerNonUserCodeAttribute(_DebuggerNonUserCodeAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x428e3627, 0x2b1f, 0x302c, 0xa7, 0xe6, 0x63, 0x88, 0xcd, 0x53, 0x5e, 0x75)]
interface _DebuggableAttribute(_DebuggableAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

//enum __declspec(uuid("86343361-ce50-35ee-8bea-6f39ec8c8159"))
ENUM!{enum DebuggingModes
{
    DebuggingModes_None = 0,
    DebuggingModes_Default = 1,
    DebuggingModes_DisableOptimizations = 256,
    DebuggingModes_IgnoreSymbolStoreSequencePoints = 2,
    DebuggingModes_EnableEditAndContinue = 4,
}}

//enum __declspec(uuid("5a235286-93f1-3c18-a3ae-16d345a87a24"))
ENUM!{enum DebuggerBrowsableState {
    DebuggerBrowsableState_Never = 0,
    DebuggerBrowsableState_Collapsed = 2,
    DebuggerBrowsableState_RootHidden = 3,
}}

RIDL!{#[uuid(0xa3fc6319, 0x7355, 0x3d7d, 0x86, 0x21, 0xb5, 0x98, 0x56, 0x11, 0x52, 0xfc)]
interface _DebuggerBrowsableAttribute(_DebuggerBrowsableAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x404fafdd, 0x1e3f, 0x3602, 0xbf, 0xf6, 0x75, 0x5c, 0x00, 0x61, 0x3e, 0xd8)]
interface _DebuggerTypeProxyAttribute(_DebuggerTypeProxyAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x22fdabc0, 0xeec7, 0x33e0, 0xb4, 0xf2, 0xf3, 0xb7, 0x39, 0xe1, 0x9a, 0x5e)]
interface _DebuggerDisplayAttribute(_DebuggerDisplayAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xe19ea1a2, 0x67ff, 0x31a5, 0xb9, 0x5c, 0xe0, 0xb7, 0x53, 0x40, 0x3f, 0x6b)]
interface _DebuggerVisualizerAttribute(_DebuggerVisualizerAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

