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

use system::collections::IDictionaryEnumerator;

RIDL!{#[uuid(0x8965a22f, 0xfba8, 0x36ad, 0x81, 0x32, 0x70, 0xbb, 0xd0, 0xda, 0x45, 0x7d)]
interface IResourceReader(IResourceReaderVtbl) : IDispatch(IDispatchVtbl)
{
    fn Close() -> HRESULT,
    fn GetEnumerator(
        pRetVal: *mut *mut IDictionaryEnumerator,
    ) -> HRESULT,
}}//IEnumerable, IDisposable