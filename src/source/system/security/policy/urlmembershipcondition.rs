use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xbb7a158d, 0xdbd9, 0x3e13, 0xb1, 0x37, 0x8e, 0x61, 0xe8, 0x7e, 0x11, 0x28)]
interface _UrlMembershipCondition(_UrlMembershipConditionVtbl): IDispatch(IDispatchVtbl)  
{}}  //IMembershipCondition, IConstantMembershipCondition, IReportMatchMembershipCondition