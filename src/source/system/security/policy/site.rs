use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x90c40b4c, 0xb0d0, 0x30f5, 0xb5, 0x20, 0xfd, 0xba, 0x97, 0xbc, 0x31, 0xa0)]
interface _Site(_SiteVtbl): IDispatch(IDispatchVtbl)  
{}} //EvidenceBase, IIdentityPermissionFactory