// iapplicationtrustmanager.rs - MIT License
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

use winapi::shared::winerror::HRESULT;

use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::um::unknwnbase::IUnknown;

use system::security::policy::_ApplicationTrust;

RIDL!{#[uuid(0x427e255d, 0xaf02, 0x3b0d, 0x8c, 0xe3, 0xa2, 0xbb, 0x94, 0xba, 0x30, 0x0f)]
interface IApplicationTrustManager(IApplicationTrustManagerVtbl): IDispatch(IDispatchVtbl)  
{
    fn DetermineApplicationTrust(
        activationContext: *mut IUnknown,
        Context: *mut _TrustManagerContext,
        pRetVal: *mut *mut _ApplicationTrust,
    ) -> HRESULT,
}}

//enum __declspec(uuid("940b1725-f706-3cef-9586-0f189b117c20"))
ENUM!{enum TrustManagerUIContext {
    TrustManagerUIContext_Install = 0,
    TrustManagerUIContext_Upgrade = 1,
    TrustManagerUIContext_Run = 2,
}}

RIDL!{#[uuid(0xd89eac5e, 0x0331, 0x3fcd, 0x9c, 0x16, 0x4f, 0x1e, 0xd3, 0xfe, 0x1b, 0xe2)]
interface _TrustManagerContext(_TrustManagerContextVtbl): IDispatch(IDispatchVtbl)  
{}}