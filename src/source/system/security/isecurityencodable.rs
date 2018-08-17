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

use system::security::securityelement::_SecurityElement;

RIDL!{#[uuid(0xfd46bde5, 0xacdf, 0x3ca5, 0xb1, 0x89, 0xf0, 0x67, 0x83, 0x87, 0x07, 0x7f)]
interface ISecurityEncodable(ISecurityEncodableVtbl): IDispatch(IDispatchVtbl)  
{
    fn ToXml(
		pRetVal: *mut *mut  _SecurityElement ,
	) -> HRESULT,
    fn FromXml(
        e: *mut _SecurityElement,
    ) -> HRESULT,
}}