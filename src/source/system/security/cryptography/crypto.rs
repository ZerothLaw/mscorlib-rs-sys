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

//enum __declspec(uuid("75c9e85e-d2d1-32db-bf9c-0636f94fb0c2"))
ENUM!{enum CipherMode
{
    CipherMode_CBC = 1,
    CipherMode_ECB = 2,
    CipherMode_OFB = 3,
    CipherMode_CFB = 4,
    CipherMode_CTS = 5,
}}

//enum __declspec(uuid("1254089d-0104-3bfb-b6ba-9168f994dca6"))
ENUM!{enum PaddingMode
{
    PaddingMode_None = 1,
    PaddingMode_PKCS7 = 2,
    PaddingMode_Zeros = 3,
    PaddingMode_ANSIX923 = 4,
    PaddingMode_ISO10126 = 5,
}}

RIDL!{#[uuid(0x8978b0be, 0xa89e, 0x3ff9, 0x98, 0x34, 0x77, 0x86, 0x2c, 0xeb, 0xff, 0x3d)]
interface _KeySizes(_KeySizesVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x4311e8f5, 0xb249, 0x3f81, 0x8f, 0xf4, 0xcf, 0x85, 0x3d, 0x85, 0x30, 0x6d)]
interface _CryptographicException(_CryptographicExceptionVtbl): IDispatch(IDispatchVtbl)  
{}} //SystemException

RIDL!{#[uuid(0x7fb08423, 0x038f, 0x3acc, 0xb6, 0x00, 0xe6, 0xd0, 0x72, 0xba, 0xe1, 0x60)]
interface _CryptographicUnexpectedOperationException(_CryptographicUnexpectedOperationExceptionVtbl): IDispatch(IDispatchVtbl)  
{}} //CryptographicException
