// contextproperty.rs - MIT License
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
use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

use crate::system::runtime::remoting::{IConstructionCallMessage,IConstructionReturnMessage,_Context};

RIDL!{#[uuid(0x4a68baa3, 0x27aa, 0x314a, 0xbd, 0xbb, 0x6a, 0xe9, 0xbd, 0xfc, 0x04, 0x20)]
interface IContextAttribute(IContextAttributeVtbl): IDispatch(IDispatchVtbl)  
{
    fn IsContextOk(
        ctx: *mut _Context, 
        msg: *mut IConstructionCallMessage, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetPropertiesForNewContext(
        msg: *mut IConstructionCallMessage,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xf01d896d, 0x8d5f, 0x3235, 0xbe, 0x59, 0x20, 0xe1, 0xe1, 0x0d, 0xc2, 0x2a)]
interface IContextProperty(IContextPropertyVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_name(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn IsNewContextOK(
		newCtx: *mut  _Context ,
		pRetVal: *mut VARIANT_BOOL ,
	) -> HRESULT,
    fn Freeze(
        newContext: *mut _Context,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x7197b56b, 0x5fa1, 0x31ef, 0xb3, 0x8b, 0x62, 0xfe, 0xe7, 0x37, 0x27, 0x7f)]
interface IContextPropertyActivator(IContextPropertyActivatorVtbl): IDispatch(IDispatchVtbl)  
{
    fn IsOKToActivate(
		msg: *mut  IConstructionCallMessage ,
		pRetVal: *mut VARIANT_BOOL ,
	) -> HRESULT,
    fn CollectFromClientContext(
        msg: *mut IConstructionCallMessage,
    ) -> HRESULT,
    fn DeliverClientContextToServerContext(
		msg: *mut  IConstructionCallMessage ,
		pRetVal: *mut VARIANT_BOOL ,
	) -> HRESULT,
    fn CollectFromServerContext(
        msg: *mut IConstructionReturnMessage,
    ) -> HRESULT,
    fn DeliverServerContextToClientContext(
		msg: *mut  IConstructionReturnMessage ,
		pRetVal: *mut VARIANT_BOOL ,
	) -> HRESULT,
}}