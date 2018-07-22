use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x6ba6ea7a, 0xc9fc, 0x3e73, 0x82, 0xec, 0x18, 0xf2, 0x9d, 0x83, 0xee, 0xfd)]
interface _HashMembershipCondition(_HashMembershipConditionVtbl): IDispatch(IDispatchVtbl)  
{}} //ISerializable, IDeserializationCallback, IMembershipCondition, IReportMatchMembershipCondition 