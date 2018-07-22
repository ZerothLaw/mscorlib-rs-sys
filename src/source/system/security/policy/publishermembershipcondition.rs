use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x3515cf63, 0x9863, 0x3044, 0xb3, 0xe1, 0x21, 0x0e, 0x98, 0xef, 0xc7, 0x02)]
interface _PublisherMembershipCondition(_PublisherMembershipConditionVtbl): IDispatch(IDispatchVtbl)  
{}} //IMembershipCondition, IConstantMembershipCondition, IReportMatchMembershipCondition