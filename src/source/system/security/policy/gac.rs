use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xa7aef52c, 0xb47b, 0x3660, 0xbb, 0x3e, 0x34, 0x34, 0x7d, 0x56, 0xdb, 0x46)]
interface _GacInstalled(_GacInstalledVtbl): IDispatch(IDispatchVtbl)  
{}} //EvidenceBase, IIdentityPermissionFactory