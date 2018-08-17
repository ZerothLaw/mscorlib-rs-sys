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

use system::runtime::remoting::ilease::ILease;
use system::timespan::TimeSpan;

RIDL!{#[uuid(0x675591af, 0x0508, 0x3131, 0xa7, 0xcc, 0x28, 0x7d, 0x26, 0x5c, 0xa7, 0xd6)]
interface ISponsor(ISponsorVtbl): IDispatch(IDispatchVtbl)  
{
    fn Renewal(
		lease: *mut  ILease ,
		pRetVal: *mut  TimeSpan ,
	) -> HRESULT,
}}