// trackingservices.rs - MIT License
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
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, VARIANT};

use system::runtime::remoting::objref::_ObjRef;

RIDL!{#[uuid(0x03ec7d10, 0x17a5, 0x3585, 0x9a, 0x2e, 0x05, 0x96, 0xfc, 0xac, 0x38, 0x70)]
interface ITrackingHandler(ITrackingHandlerVtbl): IDispatch(IDispatchVtbl)  
{
    fn MarshaledObject(
        obj: VARIANT, 
        ORR: *mut _ObjRef,
    ) -> HRESULT,

    fn UnmarshaledObject(
        obj: VARIANT, 
        ORR: *mut _ObjRef,
    ) -> HRESULT,
    
    fn DisconnectedObject(
        obj: VARIANT,
    ) -> HRESULT,
}}