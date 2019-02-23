// iformatterconverter.rs - MIT License
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

use winapi::ctypes::{c_double, c_float, c_long, c_short};
use winapi::shared::ntdef::{HRESULT};
use winapi::shared::wtypes::{BSTR,DATE,DECIMAL,VARIANT_BOOL};
use winapi::shared::wtypesbase::{UCHAR,ULONG,USHORT};

use winapi::um::oaidl::{IDispatch, IDispatchVtbl, VARIANT};

use crate::system::reflection::_Type;
use crate::system::TypeCode;

RIDL!{#[uuid(0xf4f5c303, 0xfad3, 0x3d0c, 0xa4, 0xdf, 0xbb, 0x82, 0xb5, 0xee, 0x30, 0x8f)]
interface IFormatterConverter(IFormatterConverterVtbl): IDispatch(IDispatchVtbl)  
{
    fn Convert(
        val: VARIANT, 
        Type_: *mut _Type,
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
    fn Convert_2(
        val: VARIANT, 
        typeCode: *mut TypeCode,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn ToBoolean(
        val: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn ToChar(
        val: VARIANT, 
        pRetVal: *mut USHORT,
    ) -> HRESULT,
    fn ToSByte(
        val: VARIANT,
        pRetVal: *mut char,
    ) -> HRESULT,
    fn ToByte(
        val: VARIANT, 
        pRetVal: *mut UCHAR,
    ) -> HRESULT,
    fn ToInt16(
        val: VARIANT,
        pRetVal: *mut c_short,
    ) -> HRESULT,
    fn ToUInt16(
        val: VARIANT, 
        pRetVal: *mut USHORT, 
    ) -> HRESULT,
    fn ToInt32(
        val: VARIANT,
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn ToUInt32(
        val: VARIANT, 
        pRetVal: *mut ULONG,
    ) -> HRESULT,
    fn ToInt64(
        val: VARIANT,
        pRetVal: *mut i64,
    ) -> HRESULT,
    fn ToUInt64(
        val: VARIANT, 
        pRetVal: *mut u64,
    ) -> HRESULT,
    fn ToSingle(
        val: VARIANT,
        pRetVal: *mut c_float,
    ) -> HRESULT,
    fn ToDouble(
        val: VARIANT,
        pRetVal: *mut c_double,
    ) -> HRESULT,
    fn ToDecimal(
        val: VARIANT,
        pRetVal: *mut DECIMAL,
    ) -> HRESULT,
    fn ToDateTime(
        val: VARIANT,
        pRetVal: *mut DATE,
    ) -> HRESULT,
    fn get_ToString(
        val: VARIANT,
        pRetVal: *mut BSTR,
    ) -> HRESULT,
}}
