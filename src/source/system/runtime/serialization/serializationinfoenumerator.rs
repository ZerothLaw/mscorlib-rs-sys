// serializationinfoenumerator.rs - MIT License
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
use winapi::shared::ntdef::LPSTR;
use winapi::um::unknwnbase::IUnknown;
use crate::system::reflection::_Type;

//struct __declspec(uuid("3642e7ed-5a69-3a94-98d3-a08877a0d046"))
STRUCT!{struct SerializationEntry
{
    m_type: *mut _Type,
    m_value: *mut IUnknown,
    m_name: LPSTR,
}}

RIDL!{#[uuid(0x607056c6, 0x1bca, 0x36c8, 0xab, 0x87, 0x33, 0xb2, 0x02, 0xeb, 0xf0, 0xd8)]
interface _SerializationInfoEnumerator(_SerializationInfoEnumeratorVtbl): IDispatch(IDispatchVtbl)  
{}} //implements IEnumerator