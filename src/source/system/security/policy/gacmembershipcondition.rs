use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xb2217ab5, 0x6e55, 0x3ff6, 0xa1, 0xa9, 0x1b, 0x0d, 0xc0, 0x58, 0x50, 0x40)]
interface _GacMembershipCondition(_GacMembershipConditionVtbl): IDispatch(IDispatchVtbl)  
{}} //IMembershipCondition, IConstantMembershipCondition, IReportMatchMembershipCondition