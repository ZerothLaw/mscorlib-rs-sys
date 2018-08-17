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
use winapi::shared::ntdef::{HRESULT};

use winapi::um::oaidl::{VARIANT};

use system::io::_Stream;

use system::runtime::serialization::_SerializationBinder;
use system::runtime::serialization::ISurrogateSelector;
use system::runtime::serialization::StreamingContext;

RIDL!{#[uuid(0x93d7a8c5, 0xd2eb, 0x319b, 0xa3, 0x74, 0xa6, 0x5d, 0x32, 0x1f, 0x2a, 0xa9)]
interface IFormatter(IFormatterVtbl): IDispatch(IDispatchVtbl)  
{
    fn Deserialize(
		serializationStream: *mut  _Stream ,
		pRetVal: *mut VARIANT ,
	) -> HRESULT,
    fn Serialize(
        serializationStream: *mut _Stream, 
        graph: VARIANT, 
    ) -> HRESULT,
    fn get_SurrogateSelector(
		pRetVal: *mut *mut  ISurrogateSelector ,
	) -> HRESULT,
    fn putref_SurrogateSelector(
        pRetVal: *mut ISurrogateSelector, 
    ) -> HRESULT,
    fn get_Binder(
		pRetVal: *mut *mut  _SerializationBinder ,
	) -> HRESULT,
    fn putref_Binder(
        pRetVal: *mut _SerializationBinder,
    ) -> HRESULT,
    fn get_Context(
        pRetVal: StreamingContext ,
    ) -> HRESULT,
    fn put_Context(
        pRetVal: StreamingContext,
    ) -> HRESULT,
}}