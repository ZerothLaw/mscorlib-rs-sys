// isecuritypolicyencodable.rs - MIT License
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

use crate::system::security::policy::_PolicyLevel;
use crate::system::security::securityelement::_SecurityElement;

RIDL!{#[uuid(0xe6c21ba7, 0x21bb, 0x34e9, 0x8e, 0x57, 0xdb, 0x66, 0xd8, 0xce, 0x4a, 0x70)]
interface ISecurityPolicyEncodable(ISecurityPolicyEncodableVtbl): IDispatch(IDispatchVtbl)  
{
    fn ToXml(
		level: *mut  _PolicyLevel ,
		pRetVal: *mut *mut  _SecurityElement ,
	) -> HRESULT,
    fn FromXml(
        e: *mut _SecurityElement,
        level: *mut _PolicyLevel,
    ) -> HRESULT,
}}