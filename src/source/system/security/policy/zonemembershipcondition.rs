use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xadbc3463, 0x0101, 0x3429, 0xa0, 0x6c, 0xdb, 0x2f, 0x1d, 0xd6, 0xb7, 0x24)]
interface _ZoneMembershipCondition(_ZoneMembershipConditionVtbl): IDispatch(IDispatchVtbl)  
{}} //IMembershipCondition, IConstantMembershipCondition, IReportMatchMembershipCondition