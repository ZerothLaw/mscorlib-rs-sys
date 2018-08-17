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
use winapi::um::oaidl::SAFEARRAY;
use winapi::um::oaidl::VARIANT;

use system::runtime::remoting::_HeaderHandler;
use system::io::_Stream;

RIDL!{#[uuid(0xae1850fd, 0x3596, 0x3727, 0xa2, 0x42, 0x2f, 0xc3, 0x1c, 0x5a, 0x03, 0x12)]
interface IRemotingFormatter(IRemotingFormatterVtbl): IDispatch(IDispatchVtbl)  
{
    fn Deserialize(
        serializationStream: *mut _Stream,
        handler: *mut _HeaderHandler,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,

    fn Serialize(
        serializationStream: *mut _Stream, 
        graph: VARIANT,
        headers: *mut SAFEARRAY,
    ) -> HRESULT,
}}
