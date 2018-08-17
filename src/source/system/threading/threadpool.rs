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

RIDL!{#[uuid(0x64409425, 0xf8c9, 0x370e, 0x80, 0x9e, 0x32, 0x41, 0xce, 0x80, 0x45, 0x41)]
interface _RegisteredWaitHandle(_RegisteredWaitHandleVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xce949142, 0x4d4c, 0x358d, 0x89, 0xa9, 0xe6, 0x9a, 0x53, 0x1a, 0xa3, 0x63)]
interface _WaitCallback(_WaitCallbackVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf078f795, 0xf452, 0x3d2d, 0x8c, 0xc8, 0x16, 0xd6, 0x6a, 0xe4, 0x6c, 0x67)]
interface _WaitOrTimerCallback(_WaitOrTimerCallbackVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xbbae942d, 0xbff4, 0x36e2, 0xa3, 0xbc, 0x50, 0x8b, 0xb3, 0x80, 0x1f, 0x4f)]
interface _IOCompletionCallback(_IOCompletionCallbackVtbl): IDispatch(IDispatchVtbl)  
{}}