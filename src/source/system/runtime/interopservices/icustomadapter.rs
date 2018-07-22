use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::ntdef::{HRESULT};
use winapi::um::unknwnbase::{IUnknown};

RIDL!{#[uuid(0x3cc86595, 0xfeb5, 0x3ce9, 0xba, 0x14, 0xd0, 0x5c, 0x8d, 0xc3, 0x32, 0x1c)]
interface ICustomAdapter(ICustomAdapterVtbl) : IDispatch(IDispatchVtbl)
{
    fn GetUnderlyingObject(
        pRetVal: *mut *mut IUnknown,
    ) -> HRESULT,
}}
