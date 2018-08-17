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

use system::_MarshalByRefObject;
use system::runtime::remoting::IMessageSink;

RIDL!{#[uuid(0x6a5d38bc, 0x2789, 0x3546, 0x81, 0xa1, 0xf1, 0x0c, 0x0f, 0xb5, 0x93, 0x66)]
interface IContributeObjectSink(IContributeObjectSinkVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetObjectSink(
        obj: *mut _MarshalByRefObject,
        NextSink: *mut IMessageSink, 
        pRetVal: *mut *mut IMessageSink,
    ) -> HRESULT,
}}