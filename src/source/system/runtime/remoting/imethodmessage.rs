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

use winapi::ctypes::c_long;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::BSTR;
use winapi::um::oaidl::{VARIANT, SAFEARRAY};
use winapi::shared::wtypes::VARIANT_BOOL;

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
