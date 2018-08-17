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
use winapi::ctypes::c_long;
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::um::oaidl::SAFEARRAY;


RIDL!{#[uuid(0x8abad867, 0xf515, 0x3cf6, 0xbb, 0x62, 0x5f, 0x0c, 0x88, 0xb3, 0xbb, 0x11)]
interface ICryptoTransform(ICryptoTransformVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_InputBlockSize(
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn get_OutputBlockSize(
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn get_CanTransformMultipleBlocks(
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanReuseTransform(
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn TransformBlock(
        inputBuffer: *mut SAFEARRAY, //byte[]
        inputOffset: c_long,
        inputCount: c_long, 
        outputBuffer: *mut SAFEARRAY, //byte[]
        outputOffset: c_long,
        pRetVal: *mut c_long, 
    ) -> HRESULT,
    fn TransformFinalBlock(
        inputBuffer: *mut SAFEARRAY, //byte[]
        inputOffset: c_long,
        inputCount: c_long,
        pRetVal: *mut *mut SAFEARRAY, //byte[]
    ) -> HRESULT,
}}