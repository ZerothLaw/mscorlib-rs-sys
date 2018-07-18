use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x0a7c3542, 0x8031, 0x3593, 0x87, 0x2c, 0x78, 0xd8, 0x5d, 0x7c, 0xc2, 0x73)]
interface _SiteMembershipCondition(_SiteMembershipConditionVtbl): IDispatch(IDispatchVtbl)  
{}} //IMembershipCondition, IConstantMembershipCondition, IReportMatchMembershipCondition