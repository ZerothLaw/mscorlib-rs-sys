use winapi::um::oaidl::{VARIANT};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

use winapi::shared::winerror::HRESULT;

use source::system::collections::dictionaryentry::DictionaryEntry;

RIDL!{#[uuid(0x35d574bf, 0x7a4f, 0x3588, 0x8c, 0x19, 0x12, 0x21, 0x2a, 0x0f, 0xe4, 0xdc)]
interface IDictionaryEnumerator(IDictionaryEnumeratorVtbl) : IDispatch(IDispatchVtbl){
    fn get_key(
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,

    fn get_val(
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,

    fn get_Entry(
        pRetVal: *mut DictionaryEntry,
    ) -> HRESULT,
}} //inherits from IEnumerator