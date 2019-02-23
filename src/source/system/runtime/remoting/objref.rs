// objref.rs - MIT License
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

use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY};

use crate::system::reflection::_Type;
use crate::system::runtime::remoting::IMessageSink;

RIDL!{#[uuid(0xc09effa9, 0x1ffe, 0x3a52, 0xa7, 0x33, 0x62, 0x36, 0xcb, 0xc4, 0x5e, 0x7b)]
interface IRemotingTypeInfo(IRemotingTypeInfoVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_typeName(
        pRetVal: BSTR,
    ) -> HRESULT,
    fn put_typeName(
        pRetVal: BSTR,
    ) -> HRESULT,
    fn CanCastTo(
        fromType: *mut _Type,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x855e6566, 0x014a, 0x3fe8, 0xaa, 0x70, 0x1e, 0xac, 0x77, 0x1e, 0x3a, 0x88)]
interface IChannelInfo(IChannelInfoVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_ChannelData(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn put_ChannelData(
        pRetVal: *mut SAFEARRAY,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x2a6e91b9, 0xa874, 0x38e4, 0x99, 0xc2, 0xc5, 0xd8, 0x3d, 0x78, 0x14, 0x0d)]
interface IEnvoyInfo(IEnvoyInfoVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_EnvoySinks(
		pRetVal: *mut *mut  IMessageSink ,
	) -> HRESULT,
    fn putref_EnvoySinks (
        pRetVal: *mut IMessageSink,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x1dd3cf3d, 0xdf8e, 0x32ff, 0x91, 0xec, 0xe1, 0x9a, 0xa1, 0x0b, 0x63, 0xfb)]
interface _ObjRef(_ObjRefVtbl): IDispatch(IDispatchVtbl)  
{}}
//implements IObjectReference, ISerializable
