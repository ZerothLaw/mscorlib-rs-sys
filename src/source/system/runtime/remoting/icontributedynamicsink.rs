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

use system::runtime::remoting::IDynamicMessageSink;

RIDL!{#[uuid(0xa0fe9b86, 0x0c06, 0x32ce, 0x85, 0xfa, 0x2f, 0xf1, 0xb5, 0x86, 0x97, 0xfb)]
interface IContributeDynamicSink(IContributeDynamicSinkVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetDynamicSink(
		pRetVal: *mut *mut  IDynamicMessageSink ,
	) -> HRESULT,
}}
