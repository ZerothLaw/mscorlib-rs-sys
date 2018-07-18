use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x579e93bc, 0xffab, 0x3b8d, 0x91, 0x81, 0xce, 0x9c, 0x22, 0xb5, 0x19, 0x15)]
interface _StrongNameMembershipCondition(_StrongNameMembershipConditionVtbl): IDispatch(IDispatchVtbl)  
{}}  // IMembershipCondition, IConstantMembershipCondition, IReportMatchMembershipCondition