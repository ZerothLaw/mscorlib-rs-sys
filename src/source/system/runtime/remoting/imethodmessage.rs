// imethodmessage.rs - MIT License
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

use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::{BSTR,VARIANT_BOOL};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, VARIANT, SAFEARRAY};

use system::_Exception;
use system::reflection::_MethodBase;
use system::runtime::remoting::_LogicalCallContext;

RIDL!{#[uuid(0x8e5e0b95, 0x750e, 0x310d, 0x89, 0x2c, 0x8c, 0xa7, 0x23, 0x1c, 0xf7, 0x5b)]
interface IMethodMessage(IMethodMessageVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_Uri(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_MethodName(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_typeName(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_MethodSignature(
        pRetVal: VARIANT ,
    ) -> HRESULT,
    fn get_ArgCount(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn GetArgName(
        index: c_long,
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn GetArg(
        argNum: c_long,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn get_args(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn get_HasVarArgs(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_LogicalCallContext(
		pRetVal: *mut *mut  _LogicalCallContext ,
	) -> HRESULT,
    fn get_MethodBase(
		pRetVal: *mut *mut  _MethodBase ,
	) -> HRESULT,
}} //IMessage


RIDL!{#[uuid(0xb90efaa6, 0x25e4, 0x33d2, 0xac, 0xa3, 0x94, 0xbf, 0x74, 0xdc, 0x4a, 0xb9)]
interface IMethodCallMessage(IMethodCallMessageVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_InArgCount(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn GetInArgName(
        index: c_long,
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn GetInArg(
        argNum: c_long,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn get_InArgs(
	    pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
}}

RIDL!{#[uuid(0xf617690a, 0x55f4, 0x36af, 0x91, 0x49, 0xd1, 0x99, 0x83, 0x1f, 0x85, 0x94)]
interface IMethodReturnMessage(IMethodReturnMessageVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_OutArgCount(
        pRetVal: c_long,
    ) -> HRESULT,
    fn GetOutArgName(
        index: c_long,
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn GetOutArg(
        argNum: c_long,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn get_OutArgs(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn get_Exception(
		pRetVal: *mut *mut  _Exception ,
	) -> HRESULT,
    fn get_ReturnValue(
        pRetVal: VARIANT ,
    ) -> HRESULT,
}}
