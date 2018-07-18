use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;

use source::system::runtime::remoting::leasestate::LeaseState;
use source::system::timespan::TimeSpan;
use source::system::runtime::remoting::isponsor::ISponsor;

RIDL!{#[uuid(0x53a561f2, 0xcbbf, 0x3748, 0xbf, 0xfe, 0x21, 0x80, 0x00, 0x2d, 0xb3, 0xdf)]
interface ILease(ILeaseVtbl): IDispatch(IDispatchVtbl)  
{
    fn Register(
        obj: *mut ISponsor,
        renewalTime: TimeSpan, 
    ) -> HRESULT,
    fn Register_2(
        obj: *mut ISponsor,
    ) -> HRESULT,
    fn Unregister(
        obj: *mut ISponsor, 
    ) -> HRESULT,
    fn Renew(
        renewalTime: TimeSpan, 
        pRetVal: *mut TimeSpan,
    ) -> HRESULT,
    fn get_RenewOnCallTime(
        pRetVal: TimeSpan ,
    ) -> HRESULT,
    fn put_RenewOnCallTime(
        pRetVal: TimeSpan,
    ) -> HRESULT,
    fn get_SponsorshipTimeout(
        pRetVal:  TimeSpan ,
    ) -> HRESULT,
    fn put_SponsorshipTimeout(
        pRetVal: TimeSpan,
    ) -> HRESULT,
    fn get_InitialLeaseTime(
        pRetVal: TimeSpan ,
    ) -> HRESULT,
    fn put_InitialLeaseTime(
        pRetVal: TimeSpan,
    ) -> HRESULT,
    fn get_CurrentLeaseTime(
        pRetVal:  TimeSpan ,
    ) -> HRESULT,
    fn get_CurrentState(
        pRetVal: LeaseState ,
    ) -> HRESULT,
}}