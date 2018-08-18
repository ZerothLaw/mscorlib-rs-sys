// remotingconfiguration.rs - MIT License
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

RIDL!{#[uuid(0x4b10971e, 0xd61d, 0x373f, 0xbc, 0x8d, 0x2c, 0xcf, 0x31, 0x12, 0x62, 0x15)]
interface _RemotingConfiguration(_RemotingConfigurationVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x8359f3ab, 0x643f, 0x3bcf, 0x91, 0xe8, 0x16, 0xe7, 0x79, 0xed, 0xeb, 0xe1)]
interface _TypeEntry(_TypeEntryVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xbac12781, 0x6865, 0x3558, 0xa8, 0xd1, 0xf1, 0xca, 0xdd, 0x28, 0x06, 0xdd)]
interface _ActivatedClientTypeEntry(_ActivatedClientTypeEntryVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x94855a3b, 0x5ca2, 0x32cf, 0xb1, 0xab, 0x48, 0xfd, 0x39, 0x15, 0x82, 0x2c)]
interface _ActivatedServiceTypeEntry(_ActivatedServiceTypeEntryVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x4d0bc339, 0xe3f9, 0x3e9e, 0x8f, 0x68, 0x92, 0x16, 0x8e, 0x6f, 0x69, 0x81)]
interface _WellKnownClientTypeEntry(_WellKnownClientTypeEntryVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x60b8b604, 0x0aed, 0x3093, 0xac, 0x05, 0xeb, 0x98, 0xfb, 0x29, 0xfc, 0x47)]
interface _WellKnownServiceTypeEntry(_WellKnownServiceTypeEntryVtbl): IDispatch(IDispatchVtbl)  
{}}

//enum __declspec(uuid("82febf4c-9fc8-3285-8d5a-f00dd1e1ba40"))
ENUM!{enum CustomErrorsModes
{
    CustomErrorsModes_On = 0,
    CustomErrorsModes_Off = 1,
    CustomErrorsModes_RemoteOnly = 2,
}}