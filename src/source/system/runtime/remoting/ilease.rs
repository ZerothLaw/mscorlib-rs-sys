// ilease.rs - MIT License
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

use system::TimeSpan;
use system::runtime::remoting::{LeaseState,ISponsor};

RIDL!{#[uuid(0x53a561f2, 0xcbbf, 0x3748, 0xbf, 0xfe, 0x21, 0x80, 0x00, 0x2d, 0xb3, 0xdf)]
interface ILease(ILeaseVtbl): IDispatch(IDispatchVtbl)  
{
    fn Register(
        obj: *mut ISponsor,
        renewalTime: TimeSpan, 
    ) -> HRESULT,
    fn Register_2(
        obj: *mut ISponsor,
    ) -> HRESULT,
    fn Unregister(
        obj: *mut ISponsor, 
    ) -> HRESULT,
    fn Renew(
        renewalTime: TimeSpan, 
        pRetVal: *mut TimeSpan,
    ) -> HRESULT,
    fn get_RenewOnCallTime(
        pRetVal: TimeSpan ,
    ) -> HRESULT,
    fn put_RenewOnCallTime(
        pRetVal: TimeSpan,
    ) -> HRESULT,
    fn get_SponsorshipTimeout(
        pRetVal:  TimeSpan ,
    ) -> HRESULT,
    fn put_SponsorshipTimeout(
        pRetVal: TimeSpan,
    ) -> HRESULT,
    fn get_InitialLeaseTime(
        pRetVal: TimeSpan ,
    ) -> HRESULT,
    fn put_InitialLeaseTime(
        pRetVal: TimeSpan,
    ) -> HRESULT,
    fn get_CurrentLeaseTime(
        pRetVal:  TimeSpan ,
    ) -> HRESULT,
    fn get_CurrentState(
        pRetVal: LeaseState ,
    ) -> HRESULT,
}}