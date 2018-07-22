use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;

use system::security::policy::evidence::_Evidence;

RIDL!{#[uuid(0x35a8f3ac, 0xfe28, 0x360f, 0xa0, 0xc0, 0x9a, 0x4d, 0x50, 0xc4, 0x68, 0x2a)]
interface IEvidenceFactory(IEvidenceFactoryVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_Evidence(
		pRetVal: *mut *mut  _Evidence ,
	) -> HRESULT,
}}
