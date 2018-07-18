use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::ntdef::{HRESULT};

use winapi::um::oaidl::{VARIANT};

RIDL!{#[uuid(0xab3f47e4, 0xc227, 0x3b05, 0xbf, 0x9f, 0x94, 0x64, 0x9b, 0xef, 0x98, 0x88)]
interface IDeserializationCallback(IDeserializationCallbackVtbl) : IDispatch(IDispatchVtbl)
{
    fn OnDeserialization(
        sender: VARIANT, 
    ) -> HRESULT,
}}