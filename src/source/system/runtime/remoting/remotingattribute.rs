// remotingattribute.rs - MIT License
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

//enum __declspec(uuid("c888351b-5dfd-3a9f-8d36-96e7770d0ebf"))
ENUM!{enum SoapOption
{
    SoapOption_None = 0,
    SoapOption_AlwaysIncludeTypes = 1,
    SoapOption_XsdString = 2,
    SoapOption_EmbedAll = 4,
    SoapOption_Option1 = 8,
    SoapOption_Option2 = 16,
}}

//enum __declspec(uuid("0ad279c7-05fb-3a46-9031-92e00c9f7c29"))
ENUM!{enum XmlFieldOrderOption
{
    XmlFieldOrderOption_All = 0,
    XmlFieldOrderOption_Sequence = 1,
    XmlFieldOrderOption_Choice = 2,
}}

RIDL!{#[uuid(0xebcdcd84, 0x8c74, 0x39fd, 0x82, 0x1c, 0xf5, 0xeb, 0x3a, 0x27, 0x04, 0xd7)]
interface _SoapTypeAttribute(_SoapTypeAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc58145b5, 0xbd5a, 0x3896, 0x95, 0xd9, 0xb3, 0x58, 0xf5, 0x4f, 0xbc, 0x44)]
interface _SoapMethodAttribute(_SoapMethodAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x46a3f9ff, 0xf73c, 0x33c7, 0xbc, 0xc3, 0x1b, 0xef, 0x4b, 0x25, 0xe4, 0xae)]
interface _SoapFieldAttribute(_SoapFieldAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc32abfc9, 0x3917, 0x30bf, 0xa7, 0xbc, 0x44, 0x25, 0x0b, 0xdf, 0xc5, 0xd8)]
interface _SoapParameterAttribute(_SoapParameterAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x725692a5, 0x9e12, 0x37f6, 0x91, 0x1c, 0xe3, 0xda, 0x77, 0xe5, 0xfa, 0xca)]
interface _SoapAttribute(_SoapAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}
