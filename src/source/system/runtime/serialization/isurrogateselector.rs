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

use system::runtime::serialization::iserializationsurrogate::ISerializationSurrogate;
use system::runtime::serialization::streamingcontext::StreamingContext;
use system::reflection::_Type;

RIDL!{#[uuid(0x7c66ff18, 0xa1a5, 0x3e19, 0x85, 0x7b, 0x0e, 0x7b, 0x6a, 0x9e, 0x3f, 0x38)]
interface ISurrogateSelector(ISurrogateSelectorVtbl): IDispatch(IDispatchVtbl)  
{
    fn ChainSelector(
        selector: *mut ISurrogateSelector,
    ) -> HRESULT,
    fn GetSurrogate(
        Type_: *mut _Type, 
        Context: StreamingContext, 
        selector: *mut *mut ISurrogateSelector, 
        pRetVal: *mut *mut ISerializationSurrogate,
    ) -> HRESULT,
    fn GetNextSelector(
		pRetVal: *mut *mut  ISurrogateSelector ,
	) -> HRESULT,
}}