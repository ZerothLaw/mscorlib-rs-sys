use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x2a75c1fd, 0x06b0, 0x3cbb, 0xb4, 0x67, 0x25, 0x45, 0xd4, 0xd6, 0xc8, 0x65)]
interface _StrongName(_StrongNameVtbl): IDispatch(IDispatchVtbl)  
{}} //EvidenceBase, IIdentityPermissionFactory, IDelayEvaluatedEvidence