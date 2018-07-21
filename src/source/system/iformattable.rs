
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::wtypes::BSTR;
use system::iformatprovider::IFormatProvider;

RIDL!{#[uuid(0x9a604ee7, 0xe630, 0x3ded, 0x94, 0x44, 0xba, 0xae, 0x24, 0x70, 0x75, 0xab)]
interface IFormattable(IFormattableVtbl): IDispatch(IDispatchVtbl)  
{
    fn ToString(
        format: BSTR, 
        formatProvider: *mut IFormatProvider, 
        pRetVal: *mut BSTR, 
    ) -> HRESULT,
}}
