
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

use winapi::shared::winerror::HRESULT;

use source::system::collections::ienumerator::IEnumerator;

RIDL!{#[uuid(0x496b0abe, 0xcdee, 0x11d3, 0x88, 0xe8, 0x00, 0x90, 0x27, 0x54, 0xc4, 0x3a)]
interface IEnumerable(IEnumerableVtbl): IDispatch(IDispatchVtbl){
    fn GetEnumerator( 
        pRetVal: *mut *mut IEnumerator, 
    ) -> HRESULT,
}}