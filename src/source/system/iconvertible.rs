// iconvertible.rs - MIT License
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

use winapi::ctypes::{c_char, c_double, c_float, c_long, c_short};

use winapi::shared::minwindef::{UCHAR, ULONG, USHORT};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::{BSTR, DATE, DECIMAL, VARIANT_BOOL};

use winapi::um::oaidl::{IDispatch, IDispatchVtbl, VARIANT};

use crate::system::IFormatProvider;
use crate::system::reflection::_Type;
use crate::system::TypeCode;

RIDL!{#[uuid(0x805e3b62, 0xb5e9, 0x393d, 0x89, 0x41, 0x37, 0x7d, 0x8b, 0xf4, 0x55, 0x6b)]
interface IConvertible(IConvertibleVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetTypeCode(
        pRetVal: *mut TypeCode,
    ) -> HRESULT,
    fn ToBoolean(
        provider: *mut IFormatProvider,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn ToChar(
        provider: *mut IFormatProvider,
        pRetVal: *mut USHORT,
    ) -> HRESULT,
    fn ToSByte(
        provider: *mut IFormatProvider,
        pRetVal: *mut c_char,
    ) -> HRESULT,
    fn ToByte(
        provider: *mut IFormatProvider,
        pRetVal: *mut UCHAR,
    ) -> HRESULT,
    fn ToInt16(
        provider: *mut  IFormatProvider,
        pRetVal: *mut c_short,
    ) -> HRESULT,
    fn ToUInt16(
        provider: *mut  IFormatProvider,
        pRetVal: *mut USHORT,
    ) -> HRESULT,
    fn ToInt32(
        provider: *mut  IFormatProvider,
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn ToUInt32(
        provider: *mut  IFormatProvider,
        pRetVal: *mut ULONG,
    ) -> HRESULT,
    fn ToInt64(
        provider: *mut  IFormatProvider,
        pRetVal: *mut i64,
    ) -> HRESULT,
    fn ToUInt64(
        provider: *mut  IFormatProvider,
        pRetVal: *mut u64,
    ) -> HRESULT,
    fn ToSingle(
        provider: *mut  IFormatProvider,
        pRetVal: *mut c_float,
    ) -> HRESULT,
    fn ToDouble(
        provider: *mut  IFormatProvider,
        pRetVal: *mut c_double,
    ) -> HRESULT,
    fn ToDecimal(
        provider: *mut  IFormatProvider,
        pRetVal: *mut DECIMAL,
    ) -> HRESULT,
    fn ToDateTime(
        provider: *mut  IFormatProvider,
        pRetVal: *mut DATE,
    ) -> HRESULT,
    fn get_ToString(
        provider: *mut  IFormatProvider,
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn ToType(
        conversionType: *mut _Type, 
        provider: *mut IFormatProvider,
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
}}