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
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::shared::wtypes::BSTR;

use system::runtime::remoting::IConstructionReturnMessage;
use system::runtime::remoting::IConstructionCallMessage;
use system::runtime::remoting::_Context;

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