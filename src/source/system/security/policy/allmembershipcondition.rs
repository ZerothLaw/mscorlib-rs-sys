use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x99f01720, 0x3cc2, 0x366d, 0x9a, 0xb9, 0x50, 0xe3, 0x66, 0x47, 0x61, 0x7f)]
interface _AllMembershipCondition(_AllMembershipConditionVtbl): IDispatch(IDispatchVtbl)  
{}} //IMembershipCondition, IConstantMembershipCondition, IReportMatchMembershipCondition