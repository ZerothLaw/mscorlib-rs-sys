use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x742e0c26, 0x0e23, 0x3d20, 0x96, 0x8c, 0xd2, 0x21, 0x09, 0x49, 0x09, 0xaa)]
interface _Zone(_ZoneVtbl): IDispatch(IDispatchVtbl)  
{}} //EvidenceBase, IIdentityPermissionFactory