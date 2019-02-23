// iactivator.rs - MIT License
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

use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::BSTR;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY};

use crate::system::collections::IList;
use crate::system::reflection::_Type;

RIDL!{#[uuid(0xc02bbb79, 0x5aa8, 0x390d, 0x92, 0x7f, 0x71, 0x7b, 0x7b, 0xff, 0x06, 0xa1)]
interface IActivator(IActivatorVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_NextActivator(
		pRetVal: *mut *mut  IActivator ,
	) -> HRESULT,
    fn putref_Activator(
        pRetVal: *mut IActivator,
    ) -> HRESULT,
    fn Activate(
		msg: *mut  IConstructionCallMessage ,
		pRetVal: *mut *mut  IConstructionReturnMessage ,
	) -> HRESULT,
    fn get_level(
        pRetVal: ActivatorLevel ,
    ) -> HRESULT,
}}

//enum __declspec(uuid("b946ac61-dd6b-39f3-bbe1-e4c1540f16ea"))
ENUM!{enum ActivatorLevel
{
    ActivatorLevel_Construction = 4,
    ActivatorLevel_Context = 8,
    ActivatorLevel_AppDomain = 12,
    ActivatorLevel_Process = 16,
    ActivatorLevel_Machine = 20,
}}

RIDL!{#[uuid(0xfa28e3af, 0x7d09, 0x31d5, 0xbe, 0xeb, 0x7f, 0x26, 0x26, 0x49, 0x7c, 0xde)]
interface IConstructionCallMessage(IConstructionCallMessageVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_Activator(
		pRetVal: *mut *mut  IActivator ,
	) -> HRESULT,
    fn putref_Activator(
        pRetVal: *mut IActivator,
    ) -> HRESULT,
    fn get_CallSiteActivationAttributes(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn get_ActivationTypeName(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_ActivationType(
		pRetVal: *mut *mut  _Type ,
	) -> HRESULT,
    fn get_ContextProperties(
		pRetVal: *mut *mut  IList ,
	) -> HRESULT,
}} //IMethodCallMessage inheritance

RIDL!{#[uuid(0xca0ab564, 0xf5e9, 0x3a7f, 0xa8, 0x0b, 0xeb, 0x0a, 0xee, 0xfa, 0x44, 0xe9)]
interface IConstructionReturnMessage(IConstructionReturnMessageVtbl): IDispatch(IDispatchVtbl)  
{}}
