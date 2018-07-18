use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
RIDL!{#[uuid(0x2b130940, 0xca5e, 0x3406, 0x83, 0x85, 0xe2, 0x59, 0xe6, 0x8a, 0xb0, 0x39)]
interface ICustomFormatter(ICustomFormatterVtbl): IDispatch(IDispatchVtbl)  
{
    fn format(
        format: BSTR, 
        arg: *mut VARIANT, 
        formatProvider: *mut IFormatProvider,
        pRetVal: *mut BSTR,
    ) -> HRESULT,
}}