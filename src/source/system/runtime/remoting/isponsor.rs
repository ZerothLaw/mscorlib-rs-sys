use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;

use system::runtime::remoting::ilease::ILease;
use system::timespan::TimeSpan;

RIDL!{#[uuid(0x675591af, 0x0508, 0x3131, 0xa7, 0xcc, 0x28, 0x7d, 0x26, 0x5c, 0xa7, 0xd6)]
interface ISponsor(ISponsorVtbl): IDispatch(IDispatchVtbl)  
{
    fn Renewal(
		lease: *mut  ILease ,
		pRetVal: *mut  TimeSpan ,
	) -> HRESULT,
}}