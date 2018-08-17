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
use winapi::um::oaidl::VARIANT;
use winapi::shared::winerror::HRESULT;

use system::runtime::serialization::streamingcontext::StreamingContext;
use system::runtime::serialization::isurrogateselector::ISurrogateSelector;
use system::runtime::serialization::serializationinfo::_SerializationInfo;

RIDL!{#[uuid(0x62339172, 0xdbfa, 0x337b, 0x8a, 0xc8, 0x05, 0x3b, 0x24, 0x1e, 0x06, 0xab)]
interface ISerializationSurrogate(ISerializationSurrogateVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetObjectData(
        obj: VARIANT, 
        info: *mut _SerializationInfo, 
        Context: StreamingContext,
    ) -> HRESULT,
    fn SetObjectData(
        obj: VARIANT, 
        info: *mut _SerializationInfo, 
        Context: StreamingContext, 
        selector: *mut ISurrogateSelector, 
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
}}