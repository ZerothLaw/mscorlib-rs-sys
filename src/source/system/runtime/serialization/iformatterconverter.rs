use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::ntdef::{HRESULT};
use winapi::shared::wtypes::BSTR;
use winapi::um::oaidl::VARIANT;
use winapi::shared::wtypes::DATE;
use winapi::shared::wtypes::DECIMAL;
use winapi::ctypes::c_double;
use winapi::ctypes::c_float;
use winapi::shared::wtypesbase::ULONG;
use winapi::ctypes::c_long;
use winapi::shared::wtypesbase::USHORT;
use winapi::ctypes::c_short;
use winapi::shared::wtypesbase::UCHAR;
use winapi::shared::wtypes::VARIANT_BOOL;

use system::reflection::cominterfaces::_Type;
use system::typecode::TypeCode;

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
