//    Copyright 2018 Tyler Laing
// 
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
// 
//        http://www.apache.org/licenses/LICENSE-2.0
// 
//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

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
