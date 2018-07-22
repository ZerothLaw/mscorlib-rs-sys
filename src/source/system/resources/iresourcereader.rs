use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;

use system::collections::IDictionaryEnumerator;

RIDL!{#[uuid(0x8965a22f, 0xfba8, 0x36ad, 0x81, 0x32, 0x70, 0xbb, 0xd0, 0xda, 0x45, 0x7d)]
interface IResourceReader(IResourceReaderVtbl) : IDispatch(IDispatchVtbl)
{
    fn Close() -> HRESULT,
    fn GetEnumerator(
        pRetVal: *mut *mut IDictionaryEnumerator,
    ) -> HRESULT,
}}//IEnumerable, IDisposable