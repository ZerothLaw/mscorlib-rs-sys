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
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::um::oaidl::SAFEARRAY;

//enum __declspec(uuid("d7dd91c9-91e4-38e9-8ec6-37836572a66a"))
ENUM!{enum KeyNumber
{
    KeyNumber_Exchange = 1,
    KeyNumber_Signature = 2,
}}

RIDL!{#[uuid(0xbe8619cb, 0x3731, 0x3cb2, 0xa3, 0xa8, 0xcd, 0x0b, 0xfa, 0x55, 0x66, 0xec)]
interface _CspKeyContainerInfo(_CspKeyContainerInfoVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x494a7583, 0x190e, 0x3693, 0x9e, 0xc4, 0xde, 0x54, 0xdc, 0x6a, 0x84, 0xa2)]
interface ICspAsymmetricAlgorithm(ICspAsymmetricAlgorithmVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_CspKeyContainerInfo(
		pRetVal: *mut *mut  _CspKeyContainerInfo ,
	) -> HRESULT,
    fn ExportCspBlob(
        includePrivateParameters: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY, //byte[]
    ) -> HRESULT,
    fn ImportCspBlob(
        rawData: *mut SAFEARRAY, //byte[]
    ) -> HRESULT,
}}