use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
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