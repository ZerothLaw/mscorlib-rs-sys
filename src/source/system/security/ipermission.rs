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

RIDL!{#[uuid(0xa19b3fc6, 0xd680, 0x3dd4, 0xa1, 0x7a, 0xf5, 0x8a, 0x7d, 0x48, 0x14, 0x94)]
interface IPermission(IPermissionVtbl): IDispatch(IDispatchVtbl)  
{
    fn Copy(
	    pRetVal: *mut *mut IPermission ,
	) -> HRESULT,
    fn Intersect(
		Target: *mut IPermission ,
		pRetVal: *mut *mut  IPermission ,
    ) -> HRESULT,
    fn Union(
	    Target: *mut IPermission ,
		pRetVal: *mut *mut IPermission ,
	) -> HRESULT,
    fn IsSubsetOf(
	    Target: *mut  IPermission ,
		pRetVal: *mut VARIANT_BOOL ,
	) -> HRESULT,
    fn Demand() -> HRESULT,
}} //ISecurityEncodable