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

use system::runtime::serialization::serializationinfo::_SerializationInfo;
use system::runtime::serialization::streamingcontext::StreamingContext;

RIDL!{#[uuid(0xd0eeaa62, 0x3d30, 0x3ee2, 0xb8, 0x96, 0xa2, 0xf3, 0x4d, 0xda, 0x47, 0xd8)]
interface ISerializable(ISerializableVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetObjectData(
        info: *mut _SerializationInfo,
        Context: StreamingContext,
    ) -> HRESULT,
}}