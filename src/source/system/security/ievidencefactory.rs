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
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

use system::security::policy::_Evidence;

RIDL!{#[uuid(0x35a8f3ac, 0xfe28, 0x360f, 0xa0, 0xc0, 0x9a, 0x4d, 0x50, 0xc4, 0x68, 0x2a)]
interface IEvidenceFactory(IEvidenceFactoryVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_Evidence(
		pRetVal: *mut *mut  _Evidence ,
	) -> HRESULT,
}}
