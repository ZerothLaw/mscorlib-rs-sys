// MIT License
// Copyright (c) 2018 Tyler Laing (ZerothLaw)

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//.Net Framework 4.7.2 Reference Source - mscorlib { system.collections.icollection.cs }

use winapi::ctypes::c_long;
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::um::oaidl::{VARIANT};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

use system::_Array;

RIDL!{#[uuid(0xde8db6f8, 0xd101, 0x3a92, 0x8d, 0x1c, 0xe7, 0x2e, 0x5f, 0x10, 0xe9, 0x92)]
interface ICollection(ICollectionVtbl): IDispatch(IDispatchVtbl)  
{
    fn CopyTo(
        Array: *mut _Array,
        index: c_long,
    ) -> HRESULT,
    fn get_Count(
        pRetVal: *mut c_long ,
    ) -> HRESULT,
    fn get_SyncRoot(
        pRetVal: *mut VARIANT ,
    ) -> HRESULT,
    fn get_IsSynchronized(
        pRetVal: *mut VARIANT_BOOL ,
    ) -> HRESULT,
}}//inherits from IEnumerable