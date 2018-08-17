// ipermission.rs - MIT License
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
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xa19b3fc6, 0xd680, 0x3dd4, 0xa1, 0x7a, 0xf5, 0x8a, 0x7d, 0x48, 0x14, 0x94)]
interface IPermission(IPermissionVtbl): IDispatch(IDispatchVtbl)  
{
    fn Copy(
	    pRetVal: *mut *mut IPermission ,
	) -> HRESULT,
    fn Intersect(
		Target: *mut IPermission ,
		pRetVal: *mut *mut  IPermission ,
    ) -> HRESULT,
    fn Union(
	    Target: *mut IPermission ,
		pRetVal: *mut *mut IPermission ,
	) -> HRESULT,
    fn IsSubsetOf(
	    Target: *mut  IPermission ,
		pRetVal: *mut VARIANT_BOOL ,
	) -> HRESULT,
    fn Demand() -> HRESULT,
}} //ISecurityEncodable