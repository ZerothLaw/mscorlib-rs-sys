// icspasymmetricalgorithm.rs - MIT License
//  MIT License
//  Copyright (c) 2018 Tyler Laing (ZerothLaw)
// 
//  Permission is hereby granted, free of charge, to any person obtaining a copy
//  of this software and associated documentation files (the "Software"), to deal
//  in the Software without restriction, including without limitation the rights
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//  copies of the Software, and to permit persons to whom the Software is
//  furnished to do so, subject to the following conditions:
// 
//  The above copyright notice and this permission notice shall be included in all
//  copies or substantial portions of the Software.
// 
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//  SOFTWARE.

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